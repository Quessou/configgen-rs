use std::error::Error;
use std::fs::{create_dir, File};
use std::path::PathBuf;
use std::io::{BufWriter, Write};

use crate::DefaultConfig;
use crate::SerializationFormat;

use serde::Serialize;
#[cfg(feature = "json")]
use serde_json;
#[cfg(feature = "json5")]
use json5_rs;
#[cfg(feature = "toml")]
use toml;
#[cfg(feature = "ron")]
use ron;

/// Creates the configuration directory at `dir_to_create` path
///
/// # Arguments
/// * `dir_to_create` - A PathBuf containing the path to the configuration dir to create
///
/// # Returns
/// * Ok(()) if the creation went fine
/// * Err(std::io::ErrorKind::AlreadyExists) if the directory already exists
/// * The error returned by `std::fs::create_dir` if it fails
pub fn create_config_dir(dir_to_create: PathBuf) -> Result<(), std::io::Error> {
    if dir_to_create.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Config directory already exists",
        ));
    }

    create_dir(dir_to_create)
}


/// Creates the configuration directory at `dir_to_create` path
///
/// # Arguments
/// * `config` - The default config to serialize
/// * `config_file_path` - The path to the file where we want to save the configuration
/// * `format` - a `SerializationFormat` value to tell which file format to use
///
/// # Returns
/// * Ok(()) if the serialization and the saving went fine 
/// * Err(std::io::ErrorKind::AlreadyExists) if the file already exists
/// * Err(std::Box(std::io::ErrorKind::Unsupported)) if the format specified is not handled by one
/// of the enabled features
/// * Any error that is returned by `BufWriter::write` if the writing in the file fails
pub fn initialize_config_file(
    config: &(impl DefaultConfig + Serialize),
    config_file_path: &PathBuf, 
    format: SerializationFormat,
) -> Result<(), std::io::Error> {
    if config_file_path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists"));
    }

    let data : Result<String, Box<dyn Error>> = match format {
        #[cfg(feature = "json")]
        SerializationFormat::Json =>  { match serde_json::to_string(&config) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e))
        } },
        #[cfg(feature = "json5")]
        SerializationFormat::Json5 => { match json5_rs::to_string(&config) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e))
        } },
        #[cfg(feature = "toml")]
        SerializationFormat::Toml => { match toml::to_string(&config){
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e))
        } }, 
        #[cfg(feature = "ron")]
        SerializationFormat::Ron => { match ron::to_string(&config) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e))
        } },
        #[allow(unreachable_patterns)]
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Unsupported, "Could not serialize the default configuration (Haven't you forgot to enable the required feature ?)")))
    };

    if data.is_err() {
       return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Parsing failed") )
    }
    let data = data.unwrap();

    let mut writer : BufWriter<File> = BufWriter::new(File::create(config_file_path).unwrap());
    match writer.write(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use serde::Deserialize;
    use temp_dir::TempDir;
    use super::*;
      
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct DummyConfig {
        pub toto: i32,
        pub tata: i64,
        pub s : String
    }

    fn get_test_init_data() -> (TempDir, PathBuf, DummyConfig) {
        let tmpdir: TempDir = TempDir::new().unwrap();
        let config_file_path = tmpdir.path().join("config");
        let dummy_config = DummyConfig { toto : 2, tata: 3, s : "test".to_owned()};

        (tmpdir, config_file_path, dummy_config)
    }

    fn read_configuration(config_file_path : &PathBuf) -> String {
        let file_to_read = std::fs::File::open(&config_file_path).unwrap();
        let mut reader = std::io::BufReader::new(file_to_read);
        let mut buf = String::new();
        let r =  reader.read_to_string(&mut buf);
        assert!(r.is_ok());
        buf
    }

    #[test]
    pub fn test_initialize_config_file_toml() {

        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Toml);
        assert!(r.is_ok());

        let config : String = read_configuration(&config_file_path);

        let read_config : DummyConfig =  toml::from_str(&config).unwrap();

    assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_initialize_config_file_json() {

        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Json);
        assert!(r.is_ok());

        let config : String = read_configuration(&config_file_path);

        let read_config : DummyConfig =  serde_json::from_str(&config).unwrap();

    assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_initialize_config_file_json5() {

        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Json5);
        assert!(r.is_ok());

        let config : String = read_configuration(&config_file_path);

        let read_config : DummyConfig =  json5_rs::from_str(&config).unwrap();

    assert_eq!(read_config, dummy_config);
    }
    #[test]
    pub fn test_initialize_config_file_ron() {

        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Ron);
        assert!(r.is_ok());

        let config : String = read_configuration(&config_file_path);

        let read_config : DummyConfig =  ron::from_str(&config).unwrap();

    assert_eq!(read_config, dummy_config);
    }
}
