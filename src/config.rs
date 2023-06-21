use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub dir: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub sender: String,
    pub relay: String
}

impl Config {
    pub fn default() -> Config {
        Config {
            url: format!("database_url"),
            dir: format!("~/.config/newsman/newsletters/"),
            smtp_username: format!("default"),
            smtp_password: format!("default"),
            sender: format!("sender"),
            relay: format!("relay"),
        }    
    }

    pub fn load_config() -> Config {
        let home: PathBuf = dirs::home_dir().expect("Cannot find home dir");
        let config_to_str: String = std::fs::read_to_string(format!("{}/.config/newsman/newsman.toml", home.display()))
            .expect("There must be a config file in .config/newsman called newsman.toml");

        let config: Result<Config, toml::de::Error> = toml::from_str(&config_to_str);

        match config {
            Ok(config) => config,
            Err(err) => panic!("Error reading config: {:?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_into_struct() {
        let config: Config = Config::load_config();

        assert_eq!("~/.config/newsman/newsletters/", config.dir)
    }
}
