#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

#[contractevent(
    topics = ["event_pass", "purchased"],
    data_format = "single-value"
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassPurchased {
    #[topic]
    pub pass_id: u32,
    #[topic]
    pub buyer: Address,
    pub purchased_at_ledger: u32,
}

#[contractevent(topics = ["event_pass", "redeemed"], data_format = "single-value")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassRedeemed {
    #[topic]
    pub pass_id: u32,
    #[topic]
    pub holder: Address,
    pub redeemed_at_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pass {
    pub holder: Address,
    pub purchased_at_ledger: u32,
    pub redeemed: bool,
    pub redeemed_at_ledger: Option<u32>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Pass(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadySold = 1,
    NotFound = 2,
    NotHolder = 3,
    AlreadyRedeemed = 4,
}

#[contract]
pub struct EventPassContract;

#[contractimpl]
impl EventPassContract {
    /// Purchases a numbered pass. The buyer must authorize the transaction.
    pub fn purchase(env: Env, buyer: Address, pass_id: u32) -> Result<Pass, Error> {
        buyer.require_auth();

        let key = DataKey::Pass(pass_id);
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadySold);
        }

        let purchased_at_ledger = env.ledger().sequence();
        let pass = Pass {
            holder: buyer.clone(),
            purchased_at_ledger,
            redeemed: false,
            redeemed_at_ledger: None,
        };

        env.storage().persistent().set(&key, &pass);
        PassPurchased {
            pass_id,
            buyer,
            purchased_at_ledger,
        }
        .publish(&env);

        Ok(pass)
    }

    /// Returns the current on-chain state for a numbered pass.
    pub fn get_pass(env: Env, pass_id: u32) -> Result<Pass, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Pass(pass_id))
            .ok_or(Error::NotFound)
    }

    /// Returns true only when the pass exists and has not been redeemed.
    pub fn is_valid(env: Env, pass_id: u32) -> bool {
        match env
            .storage()
            .persistent()
            .get::<DataKey, Pass>(&DataKey::Pass(pass_id))
        {
            Some(pass) => !pass.redeemed,
            None => false,
        }
    }

    /// Redeems the pass. Only its holder can do this, and only once.
    pub fn redeem(env: Env, holder: Address, pass_id: u32) -> Result<Pass, Error> {
        holder.require_auth();

        let key = DataKey::Pass(pass_id);
        let mut pass: Pass = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)?;

        if pass.holder != holder {
            return Err(Error::NotHolder);
        }
        if pass.redeemed {
            return Err(Error::AlreadyRedeemed);
        }

        let redeemed_at_ledger = env.ledger().sequence();
        pass.redeemed = true;
        pass.redeemed_at_ledger = Some(redeemed_at_ledger);
        env.storage().persistent().set(&key, &pass);

        PassRedeemed {
            pass_id,
            holder,
            redeemed_at_ledger,
        }
        .publish(&env);

        Ok(pass)
    }
}

#[cfg(test)]
mod test;
