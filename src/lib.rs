use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

const STATUS_CMD: &[u8] = "\x00\x06status".as_bytes();
const EOF: &str = "  \n\x00\x00";
const BUFFER_SIZE: usize = 1024;
const SEP: char = ':';

const ALL_UNITS: &[&str] = &[
  "Minutes",
  "Seconds",
  "Percent",
  "Volts",
  "Watts",
  "Amps",
  "Hz",
  "C",
  "VA",
  "Percent Load Capacity",
];

pub struct APCAccessConfig {
  pub host: String,
  pub port: u16,
  pub strip_units: bool,
}

impl Default for APCAccessConfig {
  fn default() -> Self {
    APCAccessConfig {
      host: "127.0.0.1".to_string(),
      port: 3551,
      strip_units: false,
    }
  }
}

pub struct APCAccess {
  config: APCAccessConfig,
}

impl APCAccess {
  pub fn new(config: Option<APCAccessConfig>) -> Self {
    APCAccess {
      config: config.unwrap_or_default(),
    }
  }

  pub fn fetch(&self) -> std::io::Result<HashMap<String, String>> {
    let mut stream = TcpStream::connect(format!("{}:{}", self.config.host.clone(), self.config.port))?;

    let mut output = String::new();
    stream.write_all(STATUS_CMD)?;

    loop {
      let mut buffer = [0; BUFFER_SIZE];
      let bytes_read = stream.read(&mut buffer)?;
      let data = String::from_utf8_lossy(&buffer[..bytes_read]);
      output.push_str(&data);

      if data.ends_with(EOF) {
        stream.shutdown(std::net::Shutdown::Both)?;
        break;
      }
    }

    let output = self.split_output(output);

    if self.config.strip_units {
      Ok(self.strip_units(output))
    } else {
      Ok(output)
    }
  }

  fn split_output(&self, data: String) -> HashMap<String, String> {
    data
      .replace(EOF, "")
      .split('\x00')
      .filter(|x| !x.is_empty())
      .map(|x| x[1..].to_string())
      .filter_map(|x| {
        x.split_once(SEP)
          .map(|x| (x.0.trim().to_string(), x.1.trim().to_string()))
      })
      .collect::<HashMap<String, String>>()
  }

  fn strip_units(&self, data: HashMap<String, String>) -> HashMap<String, String> {
    data
      .into_iter()
      .map(|(k, mut v)| {
        for unit in ALL_UNITS {
          v = v.trim_end_matches(unit).trim().to_string();
        }
        (k, v)
      })
      .collect::<HashMap<String, String>>()
  }
}
