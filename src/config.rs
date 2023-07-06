use serde::{Serialize, Deserialize};
use log::debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub dir: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub sender: String,
    pub relay: String,
    pub interval: u64,
    pub api_endpoint: String
}

impl Config {
    pub fn default() -> Config {
        Config {
            url: format!("mysql://root:password@localhost/newsman"),
            dir: format!("~/.config/newsman/newsletters/"),
            smtp_username: format!("newsletter@example.com"),
            smtp_password: format!("example"),
            sender: format!("newsletter@example.com"),
            relay: format!("mail.example.com"),
            interval: 0,
            api_endpoint: format!("http://127.0.0.1:3600/api/remove/")
        }    
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_smtp_username(&mut self, smtp_username: String) {
        self.smtp_username = smtp_username.clone();
        self.sender = smtp_username;
    }

    pub fn set_smtp_password(&mut self, smtp_password: String) {
        self.smtp_password = smtp_password;
    }

    pub fn set_relay(&mut self, relay: String) {
        self.relay = relay;
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
    }

    pub fn load_config() -> Result<Config, ()> {
        let config_to_str: String = std::fs::read_to_string(format!("/etc/newsman/newsman.toml"))
            .expect("There must be a config file in /etc/newsman called newsman.toml");

        let config: Result<Config, toml::de::Error> = toml::from_str(&config_to_str);

        match config {
            Ok(config) => Ok(config),
            Err(err) => Err(debug!("Error reading config: {:?}", err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_into_struct() {
        let config: Config = Config::load_config().unwrap();

        assert_eq!("~/.config/newsman/newsletters/", config.dir)
    }
}
