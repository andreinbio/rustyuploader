use std::sync::{Mutex, Arc};
use std::path::Path;
use std::fmt::Debug;

use crate::{config, config::ConfigData};
use crate::loader;
use crate::sandbox;
use crate::archive::*;

#[derive(Debug)]
pub enum Action {
    Zip,
    DeleteZip,
    SendZip,
    DeleteFolder,
    Unzip,
}

#[derive(Clone)]
pub struct Cartridge {
    name: String,
    local_path: String,
    remote_path: String,
    remote_zip_path: String,
    zip: Option<Vec<u8>>,
}

pub struct Uploader {
    config: Box<dyn ConfigData>,
    cartridges: Vec<String>,
    ignore_list: Vec<String>,
    arc_sandbox: Arc<Mutex<sandbox::Sandbox>>,
    actions: [Action;6],
}

impl Cartridge {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_local_path(&self) -> &str {
        self.local_path.as_str()
    }

    fn get_remote_path(&self) -> &str {
        self.remote_path.as_str()
    }

    fn get_remote_zip_path(&self) -> &str {
        self.remote_zip_path.as_str()
    }

    fn get_cartridge_zip(&self) -> Option<Vec<u8>> {
        self.zip.clone()
    }

    fn set_cartridge_zip(&mut self, cartridge_zip: Vec<u8>) -> () {
        self.zip = Some(cartridge_zip);
    }
}

impl Uploader {
    /// Create a new Uploader using a dw.json config file
    pub fn new_from_json<P: AsRef<Path> + Debug + Copy>(config_path: P, url: Option<String>) -> Self {
        let config: Box<dyn ConfigData> = config::parse_json(&loader::read_file(config_path));
        let cartridges: Vec<String> = if config.get_cartridges().is_empty() {
            loader::get_watched_cartridges(config.get_cartridges_path())
        } else {
            config.get_cartridges().to_owned()
        };
        let ignor_list: &[String] = config.get_ignore_list();

        let sandbox_url: String = if let Some(url) = url {
            url
        }
        else {
            format!("https://{}/on/demandware.servlet/webdav/Sites/Cartridges", config.get_hostname())
        };

        let sandbox: sandbox::Sandbox = sandbox::Sandbox::init(&config, sandbox_url.as_str());

        Uploader {
            config: config.clone(),
            cartridges: cartridges.to_vec(),
            ignore_list: ignor_list.to_vec(),
            arc_sandbox: Arc::new(Mutex::new(sandbox)),
            actions: [Action::Zip, Action::DeleteZip, Action::SendZip, Action::DeleteFolder, Action::Unzip, Action::DeleteZip],
        }
    }

    /// Create a new Uploader using an exising Config which implements ConfigData trait
    pub fn new_from_config(config: Box<dyn ConfigData>, url: Option<String>) -> Self {
        let cartridges: Vec<String> = if config.get_cartridges().is_empty() {
            loader::get_watched_cartridges(config.get_cartridges_path())
        } else {
            config.get_cartridges().to_owned()
        };
        let ignor_list: &[String] = config.get_ignore_list();

        let sandbox_url: String = if let Some(url) = url {
            url
        }
        else {
            format!("https://{}/on/demandware.servlet/webdav/Sites/Cartridges", config.get_hostname())
        };
        let sandbox: sandbox::Sandbox = sandbox::Sandbox::init(&config, sandbox_url.as_str());

        Uploader {
            config: config.clone(),
            cartridges: cartridges.to_vec(),
            ignore_list: ignor_list.to_vec(),
            arc_sandbox: Arc::new(Mutex::new(sandbox)),
            actions: [Action::Zip, Action::DeleteZip, Action::SendZip, Action::DeleteFolder, Action::Unzip, Action::DeleteZip],
        }
    }

    pub fn get_config(&self) -> &Box<dyn ConfigData> {
        &self.config
    }

    // returns active code version from the Sandbox
    pub fn get_active_codeversion(&self) -> Result<String, String> {
        self.arc_sandbox.lock().unwrap().get_active_codeversion()
    }

    pub fn get_cartridges(&self) -> &Vec<String> {
        &self.cartridges
    }

    pub fn get_actions(&self) -> &[Action;6] {
        &self.actions
    }

    // Returns cartridge data required for actions to be able to push it to sandbox
    pub fn init_cartridge_upload(&self, cartridge_name: &str) -> Option<Cartridge> {
        Some(Cartridge {
            name: cartridge_name.to_owned(),
            local_path: format!("{}/{}", self.config.get_cartridges_path(), cartridge_name),
            remote_path: format!("/{}", cartridge_name),
            remote_zip_path: format!("/{}.zip", cartridge_name),
            zip: None,
        })
    }

    pub fn push_cartridge(&self, action: &Action, cartridge_data: &mut Option<Cartridge>) -> Result<sandbox::Status, String> {
        match action {
            Action::Zip => {
                if cartridge_data.is_none() {
                    return Err("Please initialize cartridge upload first! Run 'init_cartridge_upload()'".to_owned());
                }

                let mut cartridge: Cartridge = cartridge_data.take().unwrap();
                // create cartridge zip
                let cartridge_zip = zip_dir(cartridge.get_local_path(), cartridge.get_name(), &self.ignore_list);

                if cartridge_zip.is_err() {
                    return Err(cartridge_zip.err().unwrap());
                }

                // save new cartridge zip locally
                cartridge.set_cartridge_zip(cartridge_zip.unwrap());
                cartridge_data.replace(cartridge);

                Ok(sandbox::Status::success(200, "success"))
            },
            Action::DeleteZip => {
                if cartridge_data.is_none() {
                    return Err("Please initialize cartridge upload first! Run 'init_cartridge_upload()'".to_owned());
                }

                self.arc_sandbox.lock().unwrap().delete_remote_collection(cartridge_data.clone().take().unwrap().get_remote_zip_path())
            },
            Action::SendZip => {
                if cartridge_data.is_none() {
                    return Err("Please initialize cartridge upload first! Run 'init_cartridge_upload()'".to_owned());
                }

                let cartridge: Cartridge = cartridge_data.clone().take().unwrap();
                let cartridge_zip: Option<Vec<u8>> = cartridge.get_cartridge_zip();

                if cartridge_zip.is_none() {
                    return Err("Zip cartridge is empty! It should be zipped first.".to_owned());
                }

                self.arc_sandbox.lock().unwrap().send_collection(cartridge_zip.unwrap(), cartridge.get_remote_zip_path())
            },
            Action::DeleteFolder => {
                if cartridge_data.is_none() {
                    return Err("Please initialize cartridge upload first! Run 'init_cartridge_upload()'".to_owned());
                }

                self.arc_sandbox.lock().unwrap().delete_remote_collection(cartridge_data.clone().take().unwrap().get_remote_path())
            },
            Action::Unzip => {
                if cartridge_data.is_none() {
                    return Err("Please initialize cartridge upload first! Run 'init_cartridge_upload()'".to_owned());
                }

                self.arc_sandbox.lock().unwrap().unzip_remote_zip(cartridge_data.clone().take().unwrap().get_remote_zip_path())
            },
        }
    }
}