use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::DefaultConfig;
use crate::Error as ConfiggenError;
use crate::SerializationFormat;

#[cfg(feature = "json5")]
use json5_rs;
#[cfg(feature = "ron")]
use ron;
use serde::Serialize;
#[cfg(feature = "json")]
use serde_json;
#[cfg(feature = "toml")]
use toml;

/// Creates the configuration directory at `dir_to_create` path
///
/// # Arguments
/// * `dir_to_create` - A PathBuf containing the path to the configuration dir to create
///
/// # Returns
/// * Ok(()) if the creation went fine
/// * Err(std::io::ErrorKind::AlreadyExists) if the directory already exists
/// * The error returned by `std::fs::create_dir` if it fails
pub fn create_config_dir(dir_to_create: PathBuf) -> Result<(), ConfiggenError> {
    if dir_to_create.exists() {
        let source_error = std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Config directory already exists",
        );

        return Err(ConfiggenError::ConfigDirectoryAlreadyExists(source_error));
    }

    if let Err(e) = create_dir(dir_to_create) {
        return Err(ConfiggenError::ConfigDirectoryCreationFailed(e));
    }
    Ok(())
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
) -> Result<(), ConfiggenError> {
    if config_file_path.exists() {
        let source_error =
            std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists");
        return Err(ConfiggenError::ConfigFileAlreadyExists(source_error));
    }

    let data : Result<String, Box<dyn Error + Send + Sync>> = match format {
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

    if let Err(e) = data {
        return Err(ConfiggenError::SerializationFailed(e));
    }
    let data = data.unwrap();

    let mut writer: BufWriter<File> = BufWriter::new(File::create(config_file_path).unwrap());
    match writer.write(data.as_bytes()) {
        Ok(_) => {
            writer.flush().unwrap();
            Ok(())
        }
        Err(e) => Err(ConfiggenError::WritingFailed(e)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::read_configuration;
    use config::Config;
    use serde::Deserialize;
    use temp_dir::TempDir;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct DummyConfig {
        pub toto: i32,
        pub tata: i64,
        pub s: String,
    }

    fn get_test_init_data() -> (TempDir, PathBuf, DummyConfig) {
        let tmpdir: TempDir = TempDir::new().unwrap();
        let config_file_path = tmpdir.path().join("config");
        let dummy_config = DummyConfig {
            toto: 2,
            tata: 3,
            s: "test".to_owned(),
        };

        (tmpdir, config_file_path, dummy_config)
    }

    #[test]
    pub fn test_initialize_config_file_toml() {
        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Toml);
        assert!(r.is_ok());

        let config: String = read_configuration(&config_file_path);

        let read_config: DummyConfig = toml::from_str(&config).unwrap();

        assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_initialize_config_file_json() {
        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Json);
        assert!(r.is_ok());

        let config: String = read_configuration(&config_file_path);

        let read_config: DummyConfig = serde_json::from_str(&config).unwrap();

        assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_initialize_config_file_json5() {
        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r =
            initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Json5);
        assert!(r.is_ok());

        let config: String = read_configuration(&config_file_path);

        let read_config: DummyConfig = json5_rs::from_str(&config).unwrap();

        assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_initialize_config_file_ron() {
        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Ron);
        assert!(r.is_ok());

        let config: String = read_configuration(&config_file_path);

        let read_config: DummyConfig = ron::from_str(&config).unwrap();

        assert_eq!(read_config, dummy_config);
    }

    #[test]
    pub fn test_read_config_with_config_crate() {
        let (_tmpdir, config_file_path, dummy_config) = get_test_init_data();

        let r = initialize_config_file(&dummy_config, &config_file_path, SerializationFormat::Toml);
        assert!(r.is_ok());

        let f = config::File::new(config_file_path.to_str().unwrap(), config::FileFormat::Toml);
        let config = Config::builder()
            .add_source(f)
            .build()
            .unwrap();
        let read_config = config.try_deserialize::<DummyConfig>().unwrap();
        assert_eq!(read_config, dummy_config);
    }
}
