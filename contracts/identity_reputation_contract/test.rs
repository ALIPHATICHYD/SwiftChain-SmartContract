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
fn test_register_driver_success() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    let profile = client.get_driver_profile(&driver);
    assert_eq!(profile.address, driver);
    assert_eq!(profile.deliveries_completed, 0);
    assert_eq!(profile.reputation_score, 50);
    assert_eq!(profile.kyc_verified, false);
    assert_eq!(profile.registered_at, env.ledger().timestamp());
}

#[test]
fn test_register_driver_duplicate() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    let result = client.try_register_driver(&driver);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::AlreadyInitialized.into()),
        _ => panic!("Expected duplicate driver registration to panic with AlreadyInitialized"),
    }
}

#[test]
fn test_get_driver_profile_not_found() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    let result = client.try_get_driver_profile(&driver);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::ProviderNotFound.into()),
        _ => panic!("Expected missing driver profile to return ProviderNotFound"),
    }
}

#[test]
fn test_update_kyc_status_admin_approve() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    let profile = client.get_driver_profile(&driver);
    assert_eq!(profile.kyc_verified, false);

    client.update_driver_kyc_status(&admin, &driver, &true);

    let updated = client.get_driver_profile(&driver);
    assert_eq!(updated.kyc_verified, true);
    assert_eq!(updated.address, driver);
    assert_eq!(updated.reputation_score, 50);
}

#[test]
fn test_update_kyc_status_admin_revoke() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    client.update_driver_kyc_status(&admin, &driver, &true);
    let profile = client.get_driver_profile(&driver);
    assert_eq!(profile.kyc_verified, true);

    client.update_driver_kyc_status(&admin, &driver, &false);
    let revoked = client.get_driver_profile(&driver);
    assert_eq!(revoked.kyc_verified, false);
}

#[test]
fn test_update_kyc_status_unauthorized() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    let attacker = Address::generate(&env);
    let result = client.try_update_driver_kyc_status(&attacker, &driver, &true);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::Unauthorized.into()),
        _ => panic!("Expected non-admin caller to fail with Unauthorized"),
    }
}

#[test]
fn test_update_kyc_status_driver_not_found() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    let result = client.try_update_driver_kyc_status(&admin, &driver, &true);
    match result {
        Err(Ok(err)) => assert_eq!(err, SwiftChainError::ProviderNotFound.into()),
        _ => panic!("Expected missing driver to return ProviderNotFound"),
    }
}

#[test]
fn test_update_kyc_status_persists_other_fields() {
    let (env, contract_id) = setup_env();
    let client = IdentityReputationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);

    let driver = Address::generate(&env);
    client.register_driver(&driver);

    client.update_driver_kyc_status(&admin, &driver, &true);

    let profile = client.get_driver_profile(&driver);
    assert_eq!(profile.kyc_verified, true);
    assert_eq!(profile.deliveries_completed, 0);
    assert_eq!(profile.reputation_score, 50);
    assert_eq!(profile.address, driver);
}
