Please check the codes in this dedicated branch which was forked from upstream:

https://github.com/Kailai-Wang/substrate-node-template/tree/substrate_course2

or this commit:

https://github.com/Kailai-Wang/substrate-node-template/commit/bf9976aaa36294670c8c0c833712d9764ab077bf

Specifically:

### :clipboard: Homework 2.1

please check the `sell` and `buy` functions in lib.rs

for common functions please check the private functions in `impl<T: Config> Pallet<T>`

e.g.
```
random_hash
increment_kitty_index
create_kitty_internal
transfer_internal
...
```

### :clipboard: Homework 2.2

please check the type definition of `KittyIndex` in the Config, as well as the bounding in Runtime and/or mock

### :clipboard: Homework 2.3

please check tests.rs, it should cover all the fucntions / errors / events.

to build and run the test:
```
cargo test -p pallet-kitties
```

### :clipboard: Homework 2.4

please check the usage of:

- `<pallet_balances::Pallet<T> as ReservableCurrency<_>>::reserve` in `create_kitty_internal`
- `<pallet_balances::Pallet<T> as ReservableCurrency<_>>::unreserve` in `transfer_internal`
- `pallet_balances::Pallet::<T>::transfer` in `buy`

And the unittests, especially `buy_kitty_works_with_chain_transfer`, which should illusates how the balance changes along with the transfer.

Moreover, the balance should only be unreserved once (to the original creator) even when chain transfer happens.
