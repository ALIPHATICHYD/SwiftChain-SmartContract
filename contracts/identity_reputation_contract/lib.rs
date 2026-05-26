#![no_std]

use shared_types::{SwiftChainError, UserProfile};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    UserProfile(Address),
}

#[contract]
pub struct IdentityReputationContract;

#[contractimpl]
impl IdentityReputationContract {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, SwiftChainError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, SwiftChainError::NotInitialized))
    }

    pub fn register_user(env: Env, user: Address) {
        user.require_auth();
        let key = DataKey::UserProfile(user.clone());
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, SwiftChainError::AlreadyInitialized);
        }

        let profile = UserProfile {
            address: user.clone(),
            registered_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&key, &profile);
        env.storage().persistent().extend_ttl(&key, 518400, 518400);

        env.events()
            .publish((Symbol::new(&env, "user_registered"),), (user,));
    }

    pub fn get_user_profile(env: Env, user: Address) -> UserProfile {
        let key = DataKey::UserProfile(user);
        let profile: UserProfile = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, SwiftChainError::ProviderNotFound));
        profile
    }
}

#[cfg(test)]
mod test;
