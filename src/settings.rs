use std::path::PathBuf;

use config::{Config, ConfigBuilder, ConfigError, Environment, File};
use config::builder::DefaultState;
use serde::Deserialize;

pub const DEFAULT_CONFIG_PATH: &str = "/tmp/fard.toml";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 12828;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct ParseSettings {
    input_source: String,
    output_destination: PathBuf,
    output_type: String,
    ignore_standards: Vec<String>,
    dont_segment_address: bool,
    one_address_field: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct ApiSettings {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    parse: ParseSettings,
    api: ApiSettings,
}

impl Settings {
    pub fn new(config_path: PathBuf) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(config_path.as_os_str().to_str().unwrap()))
            .add_source(Environment::with_prefix("PUB28"))
            .set_default("debug", false)?
            .set_default("parse.input_source", "stdin")?
            .set_default("parse.output_destination", "")?
            .set_default("parse.output_type", "text")?
            .set_default("parse.ignore_standards", vec![""])?
            .set_default("parse.dont_segment_address", false)?
            .set_default("parse.one_address_field", false)?
            .set_default("api.host", DEFAULT_HOST)?
            .set_default("api.port", DEFAULT_PORT)?;
        
        Ok(s)
    }
}
