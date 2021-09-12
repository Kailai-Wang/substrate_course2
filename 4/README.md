
Pleaes check codes in [`lib.rs`](./ocw-example/pallets/ocw/src/lib.rs)
or [this commit](https://github.com/Kailai-Wang/substrate_course2/commit/4554ab75866901a4a2bf787ff8be31d328cb5ae8)

More concretely, the following changes were made:
- implemented `fetch_price_info_and_send_signed` function to:
  - fetch price via http request with local cache with a validity period
  - parse the returned json to `(u64, Permill)`
  - send **signed transaction** to update the on-chain storage *Prices*
    signed transaction is used, because (see comments in codes as well):
    ```
    ///   Here signed transaction is used mainly to enforce charge of transaction fee from caller
	///   so that it's not easy (or at least costly) to attack the chain by simply spamming
	///   excesive transactions.
	///   Otherwise the unsigned validator logic would have to be extremely carefully
	///   designed for unsigned transactions to mitigate any DoS attacks.
	///   Besides that making HTTP requests to certain API too fast might result in a temp ban as well.
    ```
- refactored `fetch_n_parse` and `fetch_from_remote` so that they can be used by both *GithubInfo* and *PriceInfo*
- cargo fmt

Here are the screenshots of logs and on-chain states showing price was retrieved and persisted on chain:

![image](https://user-images.githubusercontent.com/7630809/132992174-51ea0e60-d258-4376-8221-45534e5981e6.png)

![image](https://user-images.githubusercontent.com/7630809/132992181-04becfbe-7b44-45cc-83b3-f3d0a87f4678.png)

![image](https://user-images.githubusercontent.com/7630809/132992186-c8d21b9b-70cd-40c3-9de3-c80ff2f6cbf1.png)

