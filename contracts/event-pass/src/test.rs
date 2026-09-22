use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(EventPassContract, ());
    let buyer = Address::generate(&env);

    (env, contract_id, buyer)
}

#[test]
fn purchase_and_redeem_full_flow() {
    let (env, contract_id, buyer) = setup();
    let client = EventPassContractClient::new(&env, &contract_id);

    let purchased = client.purchase(&buyer, &101);
    assert_eq!(purchased.holder, buyer);
    assert!(!purchased.redeemed);
    assert_eq!(purchased.redeemed_at_ledger, None);
    assert!(client.is_valid(&101));

    let redeemed = client.redeem(&buyer, &101);
    assert!(redeemed.redeemed);
    assert!(redeemed.redeemed_at_ledger.is_some());
    assert!(!client.is_valid(&101));

    let stored = client.get_pass(&101);
    assert_eq!(stored, redeemed);
}

#[test]
#[should_panic]
fn cannot_purchase_the_same_pass_twice() {
    let (env, contract_id, buyer) = setup();
    let client = EventPassContractClient::new(&env, &contract_id);
    client.purchase(&buyer, &202);
    client.purchase(&buyer, &202);
}

#[test]
#[should_panic]
fn only_the_holder_can_redeem() {
    let (env, contract_id, buyer) = setup();
    let client = EventPassContractClient::new(&env, &contract_id);
    let stranger = Address::generate(&env);

    client.purchase(&buyer, &303);
    client.redeem(&stranger, &303);
}

#[test]
#[should_panic]
fn a_pass_cannot_be_redeemed_twice() {
    let (env, contract_id, buyer) = setup();
    let client = EventPassContractClient::new(&env, &contract_id);

    client.purchase(&buyer, &404);
    client.redeem(&buyer, &404);
    client.redeem(&buyer, &404);
}
