use apcaccess::{APCAccess, APCAccessConfig};

pub fn main() {
  let apc = APCAccess::new(Some(APCAccessConfig {
    host: "127.0.0.1".into(),
    port: 3551,
    strip_units: true,
    timeout: 5,
  }));

  let data = apc.fetch().unwrap();

  println!("{:?}", data);
}
