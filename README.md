# apcaccess-rs

A Rust library allowing access to the data provided by apcupsd.

## Usage

```rust
use apcaccess_rs::{APCAccess, APCAccessConfig};

let apc = APCAccess::new(Some(APCAccessConfig { ..Default::default() }));
let data = apc.fetch().unwrap(); // returns a hashmap of the data
```

You can see possible keys in the [resources](resources/apcaccess-options.txt) folder based on your UPS.

`fetch()` will panic if your IP address is not valid.
