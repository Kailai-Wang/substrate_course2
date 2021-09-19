
Please check codes in [`lib.rs`](./erc20/lib.rs) and the included unittests

```
cargo test
```

![image](https://user-images.githubusercontent.com/7630809/133925355-0689fcb4-7f30-4e6a-a8b9-529950ebf37a.png)

Contract deployment and execution with canvas-UI:

Instantiation:

![image](https://user-images.githubusercontent.com/7630809/133925654-cdc56b8f-00ae-48e8-a3e1-98acecef7cee.png)

balance_of alice initially:

![image](https://user-images.githubusercontent.com/7630809/133925666-10707c32-d604-4fd9-9977-a97fe9a5aae3.png)

balance_of bob after transferring Alice -> Bob:

![image](https://user-images.githubusercontent.com/7630809/133925712-13cc60a8-5623-4edc-80b0-14499d268192.png)

transfer_from call:

![image](https://user-images.githubusercontent.com/7630809/133925832-d68dbedc-0e39-4599-9a40-dbf50a94727a.png)

balance_of Ferdie after transfer_from:

![image](https://user-images.githubusercontent.com/7630809/133925865-3b8dffc5-28d1-4878-bb17-9db7af767bf2.png)
