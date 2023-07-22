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

# Improvements
Do not hesitate to suggest improvements or report bugs on Github ! 
