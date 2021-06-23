#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod all {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    use erc20::Erc20;
    use miner_erc20::Erc20Minable;

    #[ink(storage)]
    pub struct All {
        // erc20: Lazy<Erc20>,
        // erc20_minable: Lazy<Erc20Minable>,
    }

    impl All {
        #[ink(constructor)]
        pub fn new(account1: Erc20, account2: Erc20Minable) -> Self {
            Self {
                // erc20: Lazy::new(account1),
                // erc20_minable: Lazy::new(account2),
            }
        }

        #[ink(message)]
        pub fn transfer1(&mut self, to: AccountId) {
            // (*self.erc20).transfer();
        }
    }
}