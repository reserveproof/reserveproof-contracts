#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env, Symbol, Vec};

#[contracttype]
pub enum IssuerStatus {
    Active,
    Suspended,
}

#[contracttype]
pub struct IssuerEntry {
    pub address: Address,
    pub name: Symbol,
    pub asset: Address,
    pub attestation_window_seconds: u64,
    pub required_attestors: Vec<Address>,
    pub min_signers: u32,
    pub status: IssuerStatus,
}

#[contracttype]
pub enum AttestationState {
    Pending,
    Finalized,
}

#[contracttype]
pub struct Attestation {
    pub id: BytesN<32>,
    pub issuer: Address,
    pub reserve_balance: i128,
    pub outstanding_supply: i128,
    pub supporting_doc_hash: BytesN<32>,
    pub signers: Vec<Address>,
    pub state: AttestationState,
    pub submitted_at: u64,
    pub finalized_at: Option<u64>,
}

const ADMIN_SET: Symbol = symbol_short!("admins");
const ISSUER_PREFIX: Symbol = symbol_short!("issuer");
const LATEST_ATTESTATION_PREFIX: Symbol = symbol_short!("latest");
const ATTESTATION_PREFIX: Symbol = symbol_short!("attest");

// Events
const EVENT_ATTESTATION_FINALIZED: Symbol = symbol_short!("finalized");
const EVENT_ISSUER_FLAGGED_STALE: Symbol = symbol_short!("stale");
const EVENT_ISSUER_REGISTERED: Symbol = symbol_short!("iss_reg");
const EVENT_ATTESTATION_SUBMITTED: Symbol = symbol_short!("submitted");
const EVENT_ATTESTATION_COSIGNED: Symbol = symbol_short!("cosigned");
const EVENT_ISSUER_UPDATED: Symbol = symbol_short!("iss_upd");

#[contract]
pub struct ReserveProofContract;

#[contractimpl]
impl ReserveProofContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        let admins: Vec<Address> = {
            let mut v = env.storage().instance().get(&ADMIN_SET).unwrap_or(Vec::new(&env));
            v.push_back(admin);
            v
        };
        env.storage().instance().set(&ADMIN_SET, &admins);
    }

    pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let mut admins: Vec<Address> = env.storage().instance().get(&ADMIN_SET).unwrap_or(Vec::new(&env));
        admins.push_back(new_admin);
        env.storage().instance().set(&ADMIN_SET, &admins);
    }

    pub fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let mut admins: Vec<Address> = env.storage().instance().get(&ADMIN_SET).unwrap_or(Vec::new(&env));
        let mut new_admins = Vec::new(&env);

        for admin in admins.iter() {
            if admin != admin_to_remove {
                new_admins.push_back(admin);
            }
        }

        env.storage().instance().set(&ADMIN_SET, &new_admins);
    }

    pub fn register_issuer(
        env: Env,
        caller: Address,
        issuer: Address,
        name: Symbol,
        asset: Address,
        attestation_window_seconds: u64,
        required_attestors: Vec<Address>,
        min_signers: u32,
    ) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let entry = IssuerEntry {
            address: issuer.clone(),
            name,
            asset,
            attestation_window_seconds,
            required_attestors,
            min_signers,
            status: IssuerStatus::Active,
        };

        let key = Self::issuer_key(&issuer);
        env.storage().instance().set(&key, &entry);
        env.events().publish((EVENT_ISSUER_REGISTERED,), &issuer);
    }

    pub fn update_issuer_status(env: Env, caller: Address, issuer: Address, status: IssuerStatus) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let key = Self::issuer_key(&issuer);
        let mut entry: IssuerEntry = env.storage().instance().get(&key).expect("Issuer not found");
        entry.status = status;
        env.storage().instance().set(&key, &entry);
        env.events().publish((EVENT_ISSUER_UPDATED,), &issuer);
    }

    pub fn update_attestors(
        env: Env,
        caller: Address,
        issuer: Address,
        required_attestors: Vec<Address>,
        min_signers: u32,
    ) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let key = Self::issuer_key(&issuer);
        let mut entry: IssuerEntry = env.storage().instance().get(&key).expect("Issuer not found");
        entry.required_attestors = required_attestors;
        entry.min_signers = min_signers;
        env.storage().instance().set(&key, &entry);
        env.events().publish((EVENT_ISSUER_UPDATED,), &issuer);
    }

    pub fn get_issuer(env: Env, issuer: Address) -> Option<IssuerEntry> {
        let key = Self::issuer_key(&issuer);
        env.storage().instance().get(&key)
    }

    pub fn submit_attestation(
        env: Env,
        caller: Address,
        issuer: Address,
        reserve_balance: i128,
        outstanding_supply: i128,
        supporting_doc_hash: BytesN<32>,
    ) -> BytesN<32> {
        caller.require_auth();

        let issuer_entry: IssuerEntry = env.storage().instance().get(&Self::issuer_key(&issuer)).expect("Issuer not found");

        let mut is_required = false;
        for attestor in issuer_entry.required_attestors.iter() {
            if attestor == caller {
                is_required = true;
                break;
            }
        }
        assert!(is_required, "Caller is not a required attestor");

        let mut signers = Vec::new(&env);
        signers.push_back(caller.clone());

        let mut seed_data = Bytes::new(&env);
        seed_data.append(&Bytes::from_slice(&env, &env.ledger().sequence().to_le_bytes()));
        seed_data.append(&Bytes::from_slice(&env, &env.ledger().timestamp().to_le_bytes()));
        let hash = env.crypto().sha256(&seed_data);
        let attestation_id: BytesN<32> = hash.into();
        let now = env.ledger().timestamp();

        let attestation = Attestation {
            id: attestation_id.clone(),
            issuer: issuer.clone(),
            reserve_balance,
            outstanding_supply,
            supporting_doc_hash,
            signers,
            state: if issuer_entry.min_signers <= 1 { AttestationState::Finalized } else { AttestationState::Pending },
            submitted_at: now,
            finalized_at: if issuer_entry.min_signers <= 1 { Some(now) } else { None },
        };

        let attest_key = Self::attestation_key(&attestation_id);
        env.storage().instance().set(&attest_key, &attestation);

        env.events().publish((EVENT_ATTESTATION_SUBMITTED,), (&issuer, &attestation_id));

        if issuer_entry.min_signers <= 1 {
            let latest_key = Self::latest_attestation_key(&issuer);
            env.storage().instance().set(&latest_key, &attestation_id.clone());
            env.events().publish((EVENT_ATTESTATION_FINALIZED,), (&issuer, &attestation_id));
        }

        attestation_id
    }

    pub fn co_sign_attestation(env: Env, caller: Address, attestation_id: BytesN<32>) {
        caller.require_auth();

        let attest_key = Self::attestation_key(&attestation_id);
        let mut attestation: Attestation = env.storage().instance().get(&attest_key).expect("Attestation not found");

        assert!(!matches!(attestation.state, AttestationState::Finalized), "Attestation already finalized");

        let issuer_entry: IssuerEntry = env.storage().instance().get(&Self::issuer_key(&attestation.issuer.clone())).expect("Issuer not found");

        let mut is_required = false;
        for attestor in issuer_entry.required_attestors.iter() {
            if attestor == caller {
                is_required = true;
                break;
            }
        }
        assert!(is_required, "Caller is not a required attestor");

        let mut already_signed = false;
        for signer in attestation.signers.iter() {
            if signer == caller {
                already_signed = true;
                break;
            }
        }
        assert!(!already_signed, "Caller has already signed");

        attestation.signers.push_back(caller.clone());

        env.events().publish((EVENT_ATTESTATION_COSIGNED,), (&attestation.issuer, &attestation_id, &caller));

        if attestation.signers.len() as u32 >= issuer_entry.min_signers {
            attestation.state = AttestationState::Finalized;
            attestation.finalized_at = Some(env.ledger().timestamp());

            let latest_key = Self::latest_attestation_key(&attestation.issuer);
            env.storage().instance().set(&latest_key, &attestation_id.clone());

            env.events().publish((EVENT_ATTESTATION_FINALIZED,), (&attestation.issuer, &attestation_id));
        }

        env.storage().instance().set(&attest_key, &attestation);
    }

    pub fn get_attestation(env: Env, attestation_id: BytesN<32>) -> Option<Attestation> {
        let attest_key = Self::attestation_key(&attestation_id);
        env.storage().instance().get(&attest_key)
    }

    pub fn get_latest_attestation(env: Env, issuer: Address) -> Option<Attestation> {
        let latest_key = Self::latest_attestation_key(&issuer);
        let attestation_id: Option<BytesN<32>> = env.storage().instance().get(&latest_key);

        if let Some(id) = attestation_id {
            let attest_key = Self::attestation_key(&id);
            env.storage().instance().get(&attest_key)
        } else {
            None
        }
    }

    pub fn get_reserve_ratio(env: Env, issuer: Address) -> Option<i128> {
        if let Some(attestation) = Self::get_latest_attestation(env, issuer) {
            if attestation.outstanding_supply > 0 {
                let ratio = (attestation.reserve_balance * 10000) / attestation.outstanding_supply;
                return Some(ratio);
            }
        }
        None
    }

    pub fn is_stale(env: Env, issuer: Address) -> bool {
        if let Some(attestation) = Self::get_latest_attestation(env.clone(), issuer.clone()) {
            if let Some(finalized_at) = attestation.finalized_at {
                let issuer_entry: IssuerEntry = env.storage().instance().get(&Self::issuer_key(&issuer)).expect("Issuer not found");
                let now = env.ledger().timestamp();
                return now > finalized_at + issuer_entry.attestation_window_seconds;
            }
        }
        false
    }

    pub fn flag_stale(env: Env, issuer: Address) {
        if Self::is_stale(env.clone(), issuer.clone()) {
            env.events().publish((EVENT_ISSUER_FLAGGED_STALE,), issuer);
        }
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admins: Vec<Address> = env.storage().instance().get(&ADMIN_SET).unwrap_or(Vec::new(env));

        let mut is_admin = false;
        for admin in admins.iter() {
            if admin == *caller {
                is_admin = true;
                break;
            }
        }

        assert!(is_admin, "Caller is not an admin");
    }

    fn issuer_key(issuer: &Address) -> (Symbol, Address) {
        (ISSUER_PREFIX, issuer.clone())
    }

    fn latest_attestation_key(issuer: &Address) -> (Symbol, Address) {
        (LATEST_ATTESTATION_PREFIX, issuer.clone())
    }

    fn attestation_key(id: &BytesN<32>) -> (Symbol, BytesN<32>) {
        (ATTESTATION_PREFIX, id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_set_keys() {
        // Simple test to verify key generation works
        let admin_key = ADMIN_SET;
        assert_eq!(admin_key, symbol_short!("admins"));
    }

    #[test]
    fn test_issuer_status_enum() {
        // Test enum variants are constructible
        let _active = IssuerStatus::Active;
        let _suspended = IssuerStatus::Suspended;
    }

    #[test]
    fn test_attestation_state_enum() {
        // Test enum variants are constructible
        let _pending = AttestationState::Pending;
        let _finalized = AttestationState::Finalized;
    }
}
