use std::{env, path::PathBuf};

use log::error;

pub struct Arguments {
    pub data_dir: Option<PathBuf>,
    pub config_dir: Option<PathBuf>,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            data_dir: env::current_dir().map(|p| p.join("data")).ok(),
            config_dir: env::current_dir().map(|p| p.join("config")).ok(),
        }
    }
}

impl Arguments {
    pub fn parse_or_default() -> Self {
        match Self::parse() {
            Ok(v) => v,
            Err(e) => {
                error!("Can't parse arguments {}", e);
                Default::default()
            }
        }
    }

    fn parse() -> Result<Arguments, pico_args::Error> {
        let mut parsed = pico_args::Arguments::from_env();
        let args = Arguments {
            data_dir: Some(parsed.value_from_str("--data")?),
            config_dir: Some(parsed.value_from_str("--config-dir")?),
        };
        Ok(args)
    }
}
