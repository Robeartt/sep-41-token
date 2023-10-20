//! Interface for SEP-41 Token Standard
//! https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md

#![no_std]

#[cfg(any(test, feature = "testutils"))]
pub mod testutils;

use soroban_sdk::{contractclient, symbol_short, Address, Env, String, Symbol};

/// SEP-0041 Token Standard Trait
#[contractclient(name = "TokenClient")]
pub trait Token {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`. Overrides any existing allowance set between `spender` and `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address being authorized to spend the tokens held by
    /// `from`.
    /// - `amount` - The tokens to be made available to `spender`.
    /// - `live_until_ledger` - The ledger number where this allowance expires.
    /// Cannot be less than the current ledger number unless the amount is being
    /// set to 0.  An expired entry (where live_until_ledger < the current
    /// ledger number) should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, live_until_ledger: u32]`
    ///
    /// Emits an event with:
    /// - topics - `["approve", from: Address, spender: Address]`
    /// - data - `[amount: i128, live_until_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// - `id` - The address for which a balance is being queried. If the
    /// address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be
    /// withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `[amount: i128]`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the transfer, and having its
    /// allowance consumed during the transfer.
    /// - `from` - The address holding the balance of tokens which will be
    /// withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `[amount: i128]`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be
    /// burned from.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the burn, and having its allowance
    /// consumed during the burn.
    /// - `from` - The address holding the balance of tokens which will be
    /// burned from.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    fn symbol(env: Env) -> String;
}

pub struct TokenEvents {}

impl TokenEvents {
    pub fn approve(env: &Env, from: Address, to: Address, amount: i128, expiration_ledger: u32) {
        let topics = (symbol_short!("approve"), from, to);
        env.events().publish(topics, (amount, expiration_ledger));
    }

    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        env.events().publish(topics, amount);
    }

    pub fn mint(env: &Env, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        env.events().publish(topics, amount);
    }

    pub fn clawback(env: &Env, admin: Address, from: Address, amount: i128) {
        let topics = (symbol_short!("clawback"), admin, from);
        env.events().publish(topics, amount);
    }

    pub fn set_authorized(env: &Env, admin: Address, id: Address, authorize: bool) {
        let topics = (Symbol::new(env, "set_authorized"), admin, id);
        env.events().publish(topics, authorize);
    }

    pub fn set_admin(env: &Env, admin: Address, new_admin: Address) {
        let topics = (symbol_short!("set_admin"), admin);
        env.events().publish(topics, new_admin);
    }

    pub fn burn(env: &Env, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        env.events().publish(topics, amount);
    }
}
