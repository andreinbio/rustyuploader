extern crate serde;
use serde::Deserialize;

pub trait ConfigData: ConfigDataClone {
    fn get_cartridges_path(&self) -> &str;

    fn get_hostname(&self) -> &str;

    fn get_username(&self) -> &str;

    fn get_password(&self) -> &str;

    fn get_code_version(&self) -> &str;

    fn get_cartridges(&self) -> &[String];

    fn get_ignore_list(&self) -> &[String];
}

pub trait ConfigDataClone {
    fn clone_box(&self) -> Box<dyn ConfigData>;
}

impl <T: 'static + ConfigData + Clone> ConfigDataClone for T {
    fn clone_box(&self) -> Box<dyn ConfigData> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn ConfigData> {
    fn clone(&self) -> Box<dyn ConfigData> {
        self.clone_box()
    }
}

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

impl ConfigData for Config {
    fn get_cartridges_path(&self) -> &str {
        self.cartridges_path.as_str()
    }

    fn get_hostname(&self) -> &str {
        self.hostname.as_str()
    }

    fn get_username(&self) -> &str {
        self.username.as_str()
    }

    fn get_password(&self) -> &str {
        self.password.as_str()
    }

    fn get_code_version(&self) -> &str {
        self.code_version.as_str()
    }

    fn get_cartridges(&self) -> &[String] {
        if self.cartridges.is_some() {
            return &self.cartridges.as_ref().unwrap()[..];
        }

        &[]
    }

    fn get_ignore_list(&self) -> &[String] {
        if self.ignore_list.is_some() {
            return &self.ignore_list.as_ref().unwrap()[..];
        }

        &[]
    }
}

pub fn parse_json(json: &str) ->  Box<dyn ConfigData> {
    let data: Config = serde_json::from_str(json).expect("Unable to parse json");

    Box::new(data)
}
