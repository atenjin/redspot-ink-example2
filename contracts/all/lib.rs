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
        erc20: Lazy<Erc20>,
        erc20_minable: Lazy<Erc20Minable>,
    }

    impl All {
        #[ink(constructor)]
        pub fn new(account1: Erc20, account2: Erc20Minable) -> Self {
            Self {
                erc20: Lazy::new(account1),
                erc20_minable: Lazy::new(account2),
            }
        }

        #[ink(message)]
        pub fn transfer1(&mut self, to: AccountId, value: Balance) {
            // (*self.erc20).transfer(to, value).expect("contract self do not have token")
            // (*self.erc20).transferred_value(value).transfer(to, value);
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            use ink_lang::ForwardCall;
            // let value1 = (*self.erc20).balance_of(who);
            // 等价于
            let value1 = <&Erc20>::call(&*self.erc20)
                .balance_of(who)
                .fire()
                .expect("encountered error while calling Erc20::balance_of");

            let value2 = (*self.erc20_minable).balance_of(who);
            value1 + value2
        }

        #[ink(message)]
        pub fn mine(&mut self, who: AccountId, value: Balance) {
            // (*self.erc20_minable).mine(who, value)  // 虽然mine是payable的，但是没有接口调用transfer
            // 等价于
            use ink_lang::ForwardCallMut;
            // <&mut Erc20Minable>::call_mut(&mut *self.erc20_minable)
            //     .mine(who, value)
            //     .transferred_value(value) // 加上了调用 payable 的方法的时候，提供transfer
            //     .fire()
            //     .expect("something wrong");
            self.erc20_minable.call_mut()
                .mine(who, value)
                .transferred_value(value)
                .fire()
                .expect("something wrong");
        }
    }
}