# configgen-rs
An attempt to make a small crate that generates a default configuration file on the filesystem if it does not exist yet.
Kinda thought as a very minimalistic but complementary crate for something like `config`

# Design
The crate exposes a few methods that allow to create safely a directory and a configuration file.
The one creating the configuration file takes an object implementing `serde::Serializable`, that will be serialized into the created file.

# Data formats
For now, only a few formats are handled :
* Json
* Json5
* Toml
* Ron

# How to use
```rust
use config::Config;
use configgen_rs;

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

#[derive(Serialize, Deserialize)]
struct DummyConfig {
    pub field1: i32,
}

fn main() {
    let written_config = DummyConfig { field1: 2 };
    let path = PathBuf::from_str("/tmp/test_config").unwrap();

    configgen_rs::initialization::initialize_config_file(
        &written_config,
        &path,
        configgen_rs::SerializationFormat::Toml,
    )
    .expect("Writing failed");

    // From here, testing that the config crate can read the written file.
    let f = config::File::new(path.to_str().unwrap(), config::FileFormat::Toml);
    let config = Config::builder()
        .add_source(config::File::from(f))
        .build()
        .unwrap();
    let read_config = config.try_deserialize::<DummyConfig>().unwrap();

    assert_eq!(read_config.field1, written_config.field1);
}
```

# Improvements
Do not hesitate to suggest improvements or report bugs on Github ! 
