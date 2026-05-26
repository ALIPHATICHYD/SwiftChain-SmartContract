use super::*;
use shared_types::SwiftChainError;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup_env() -> (Env, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(IdentityReputationContract, ());
    (env, contract_id)
}

#[test]
fn test_register_user_success() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let user = Address::generate(&env);
    client.register_user(&user);

    let profile = client.get_user_profile(&user);
    assert_eq!(profile.address, user);
    assert_eq!(profile.registered_at, env.ledger().timestamp());
}

#[test]
fn test_register_user_duplicate() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let user = Address::generate(&env);
    client.register_user(&user);

    let result = client.try_register_user(&user);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::AlreadyInitialized.into()),
        _ => panic!("Expected duplicate user registration to panic with AlreadyInitialized"),
    }
}

#[test]
fn test_get_user_profile_not_found() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let user = Address::generate(&env);
    let result = client.try_get_user_profile(&user);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::ProviderNotFound.into()),
        _ => panic!("Expected missing user profile to return ProviderNotFound"),
    }
}
