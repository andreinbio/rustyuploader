extern crate serde;
use serde::Deserialize;

pub trait ConfigData: ConfigDataClone {
    fn get_cartridges_path(&self) -> String;

    fn get_hostname(&self) -> String;

    fn get_username(&self) -> String;

    fn get_password(&self) -> String;

    fn get_code_version(&self) -> String;

    fn get_cartridges(&self) -> Vec<String>;

    fn get_ignore_list(&self) -> Vec<String>;
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
    fn get_cartridges_path(&self) -> String {
        self.cartridges_path.to_owned()
    }

    fn get_hostname(&self) -> String {
        self.hostname.to_owned()
    }

    fn get_username(&self) -> String {
        self.username.to_owned()
    }

    fn get_password(&self) -> String {
        self.password.to_owned()
    }

    fn get_code_version(&self) -> String {
        self.code_version.to_owned()
    }

    fn get_cartridges(&self) -> Vec<String> {
        if self.cartridges.is_some() {
            return self.cartridges.clone().take().unwrap();
        }

        vec![]
    }

    fn get_ignore_list(&self) -> Vec<String> {
        if self.ignore_list.is_some() {
            return self.ignore_list.clone().take().unwrap();
        }

        vec![]
    }
}

pub fn parse_json(json: &str) ->  Box<dyn ConfigData> {
    let data: Config = serde_json::from_str(json).expect("Unable to parse json");

    Box::new(data)
}
