#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod psp22 {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl Transfer for Psp22 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP22 for Psp22 {}

    impl Psp22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22| {
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }

        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
            self.total_supply()
        }
    }
}
