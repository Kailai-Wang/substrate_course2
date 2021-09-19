#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::{collections::HashMap, lazy::Lazy};

    #[ink(storage)]
    pub struct Erc20 {
        /// total token supply
        total_supply: Lazy<Balance>,
        /// HahsMap account owner => owned token amount
        balances: HashMap<AccountId, Balance>,
        /// HashMap (owner, spender) => allowed token amount
        /// spender is allowed to withdraw such amount of tokens from
        /// owner's account
        allowances: HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientApproval,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// init the contract with the initial total supply
        #[ink(constructor)]
        pub fn new(supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = HashMap::new();
            balances.insert(caller, supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: supply,
            });

            Self {
                total_supply: Lazy::new(supply),
                balances,
                allowances: HashMap::new(),
            }
        }

        /// getter for total_supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        /// get the owned token amount for certain `who`
        /// return 0 if account doesn't exist.
        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).copied().unwrap_or(0)
        }

        /// get the token amount which `spender` is allowed to withdraw
        /// from `owner`'s account
        /// return 0 if no allowance was set
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        }

        /// transfer `value` amount of tokens from caller's account to `to`
        /// call internal transfer func
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller(); // same as Self::env().caller();
            self.inner_transfer(from, to, value)
        }

        /// approve `to` to withdraw from caller's account for `value` token amount
        /// emit `Approval` event
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, to), value);
            self.env().emit_event(Approval {
                owner,
                spender: to,
                value,
            });
            Ok(())
        }

        /// transfers `value` tokens on the behalf of `from` to the account `to`
        ///
        /// The caller must be allowed to do so, that is:
        /// self.allowance(from, caller) must exist and sufficient allowance must be provided
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientApproval);
            }

            self.inner_transfer(from, to, value)?;
            self.allowances.insert((from, caller), allowance - value);
            Ok(())
        }

        /// internal function for transfer
        /// used by both transfer() and transfer_from()
        ///
        /// throw InsufficientBalance if not enough tokens on the `from` account
        /// when success, emit `Transfer` event
        pub fn inner_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            Ok(())
        }
    }

    /// unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        type Event = <Erc20 as ::ink_lang::BaseEvent>::Type;

        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20 = Erc20::new(100);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// The total supply was applied.
        #[ink::test]
        fn total_supply_works() {
            // Constructor works.
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Get the token total supply.
            assert_eq!(erc20.total_supply(), 100);
        }

        /// Get the actual balance of an account.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            // Alice owns all the tokens on deployment
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn transfer_fails_with_insufficient_balance() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.balance_of(accounts.bob), 0);

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob fails to transfers 10 tokens to Eve, note the caller is now bob
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                Err(Error::InsufficientBalance)
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Err(Error::InsufficientApproval)
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink_env::test::recorded_events().count(), 2);

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call.
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                Ok(())
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip checking.
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        #[ink::test]
        fn transfer_from_fails_with_insufficient_allowance() {
            // Constructor works.
            let mut erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            // Alice approves Bob for maximal 10 tokens transfer
            assert_eq!(erc20.approve(accounts.bob, 10), Ok(()));

            // The approve event takes place.
            assert_eq!(ink_env::test::recorded_events().count(), 2);

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call.
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob tries to transfer more than he is allowed (20 > 10)
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 20),
                Err(Error::InsufficientApproval)
            );

            // Alice's balance is unchanged
            assert_eq!(erc20.balance_of(accounts.alice), 100);

            // Eve doesn't have any tokens
            assert_eq!(erc20.balance_of(accounts.eve), 0);
        }

        #[ink::test]
        fn allowance_must_not_change_on_failed_transfer() {
            let mut erc20 = Erc20::new(100);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            // Alice approves Bob for token transfers on her behalf.
            let alice_balance = erc20.balance_of(accounts.alice);
            let initial_allowance = alice_balance + 2;
            assert_eq!(erc20.approve(accounts.bob, initial_allowance), Ok(()));

            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            // Create call.
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of
            data.push_arg(&accounts.bob);
            // Push the new execution context to set Bob as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // Bob tries to transfer tokens from Alice to Eve.
            let emitted_events_before = ink_env::test::recorded_events();
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
                Err(Error::InsufficientBalance)
            );
            // Allowance must have stayed the same
            assert_eq!(
                erc20.allowance(accounts.alice, accounts.bob),
                initial_allowance
            );
            // No more events must have been emitted
            let emitted_events_after = ink_env::test::recorded_events();
            assert_eq!(emitted_events_before.count(), emitted_events_after.count());
        }

        /// common assertion that are used in multiple unittests
        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("error decoding event data");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "invalid Transfer.from");
                assert_eq!(to, expected_to, "invalid Transfer.to");
                assert_eq!(value, expected_value, "invalid Trasfer.value");
            } else {
                panic!("invalid event type: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"Erc20::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"Erc20::Transfer::value",
                    value: &expected_value,
                }),
            ];
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("invalid topic encoding");
                assert_eq!(topic, expected_topic, "invalid topic at {}", n);
            }
        }

        /// For calculating the event topic hash.
        struct PrefixedValue<'a, 'b, T> {
            pub prefix: &'a [u8],
            pub value: &'b T,
        }

        impl<X> scale::Encode for PrefixedValue<'_, '_, X>
        where
            X: scale::Encode,
        {
            #[inline]
            fn size_hint(&self) -> usize {
                self.prefix.size_hint() + self.value.size_hint()
            }

            #[inline]
            fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
                self.prefix.encode_to(dest);
                self.value.encode_to(dest);
            }
        }

        fn encoded_into_hash<T>(entity: &T) -> Hash
        where
            T: scale::Encode,
        {
            use ink_env::{
                hash::{Blake2x256, CryptoHash, HashOutput},
                Clear,
            };
            let mut result = Hash::clear();
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result;
            }
            let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
    }
}
