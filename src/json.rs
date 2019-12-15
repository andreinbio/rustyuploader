extern crate serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    hostname: String,
    username: String,
    password: String,
    cartridges_path: String,
    cartridges: Option<Vec<String>>,
    ignore_list: Option<Vec<String>>,
    code_version: String,
}

impl Config {
    pub fn get_cartridges_path(&self) -> String {
        self.cartridges_path.to_owned()
    }

    pub fn get_hostname(&self) -> String {
        self.hostname.to_owned()
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }

    pub fn get_password(&self) -> String {
        self.password.to_owned()
    }

    pub fn get_code_version(&self) -> String {
        self.code_version.to_owned()
    }

    pub fn get_cartridges(&self) -> Vec<String> {
        if self.cartridges.is_some() {
            return self.cartridges.clone().take().unwrap();
        }

        vec![]
    }

    pub fn get_ignore_list(&self) -> Vec<String> {
        if self.ignore_list.is_some() {
            return self.ignore_list.clone().take().unwrap();
        }

        vec![]
    }
}

pub fn parse_config(json: &str) -> Config {
    let data: Config = serde_json::from_str(json).expect("Unable to parse json");

    data
}
