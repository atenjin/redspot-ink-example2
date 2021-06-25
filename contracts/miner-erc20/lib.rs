#![cfg_attr(not(feature = "std"), no_std)]

use ink_prelude::string::String;

pub use erc20_ownable::Erc20Minable;

#[metis_lang::contract]
mod erc20_ownable {
    use super::String;
    use erc20::Result;
    use metis_erc20 as erc20;
    use metis_lang::import;
    #[cfg(not(feature = "ink-as-dependency"))]
    use metis_lang::metis;
    // use metis_ownable as ownable;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(erc20)]
    pub struct Erc20Minable {
        erc20: erc20::Data<Erc20Minable>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::Impl<Self> for Erc20Minable{
        fn _before_token_transfer(
            &mut self,
            _: &AccountId,
            _: &AccountId,
            _: Balance,
        ) -> Result<()> {
            Ok(())
        }
    }

    // impl
    impl Erc20Minable {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
            };

            erc20::Impl::init(
                &mut instance,
                String::from("MetisTestToken"),
                String::from("MET"),
                initial_supply,
            );
            // ownable::Impl::init(&mut instance);
            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc20::Impl::name(self)
        }

        /// Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc20::Impl::symbol(self)
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            18_u8
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Impl::total_supply(self)
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Impl::balance_of(self, &owner)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Impl::allowance(self, &owner, &spender)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::transfer(self, &to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, &spender, value)
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc20::Impl::transfer_from(self, &from, &to, value)
        }

        #[ink(message, payable)]
        pub fn mine(&mut self, to: AccountId, value: Balance) {
            let enty = self.erc20.balances.entry(to);
            let old_balance = enty.or_insert(0);
            *old_balance += value;
            *self.erc20.total_supply += value;
        }
    }
}