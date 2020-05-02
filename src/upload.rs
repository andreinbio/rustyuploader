use std::sync::{Mutex, Arc};

use crate::{config, config::ConfigData};
use crate::loader;
use crate::sandbox;
use crate::archive::*;

pub struct Uploader {
    config: Box<dyn ConfigData>,
    cartridges: Vec<String>,
    ignore_list: Vec<String>,
    arc_sandbox: Arc<Mutex<sandbox::Sandbox>>,
}

impl Uploader {
    pub fn new_from_json(config_path: &str) -> Self {
        let config: Box<dyn ConfigData> = config::parse_json(&loader::read_file(config_path));
        let cartridges = if config.get_cartridges().is_empty() {
            loader::get_watched_cartridges(config.get_cartridges_path().as_str())
        } else {
            config.get_cartridges()
        };
        let ignor_list: Vec<String> = config.get_ignore_list();
        let sandbox: sandbox::Sandbox = sandbox::Sandbox::init(&config);

        Uploader {
            config: config,
            cartridges: cartridges,
            ignore_list: ignor_list,
            arc_sandbox: Arc::new(Mutex::new(sandbox)),
        }
    }

    pub fn new_from_config(config: Box<dyn ConfigData>) -> Self {
        let cartridges = if config.get_cartridges().is_empty() {
            loader::get_watched_cartridges(config.get_cartridges_path().as_str())
        } else {
            config.get_cartridges()
        };
        let ignor_list: Vec<String> = config.get_ignore_list();
        let sandbox: sandbox::Sandbox = sandbox::Sandbox::init(&config);

        Uploader {
            config: config,
            cartridges: cartridges,
            ignore_list: ignor_list,
            arc_sandbox: Arc::new(Mutex::new(sandbox)),
        }
    }

    // returns active code version from the Sandbox
    pub fn get_active_codeversion(&self) -> Result<String, String> {
        self.arc_sandbox.lock().unwrap().get_active_codeversion()
    }

    // pushes to sandbox all watched files
    pub fn push_all_files(&self) -> () {
        let cartridges_path: String = self.config.get_cartridges_path();
        for collection_name in self.cartridges.iter() {
            let collection_path: String = format!("{}/{}", cartridges_path, collection_name);
            let remote_zip_path = format!("/{}.zip", collection_name);
            let remote_folder_path = format!("/{}", collection_name);

            println!("[{}] Zipping", collection_name);
            let collection_zip = zip_dir(collection_path.as_str(), collection_name, &self.ignore_list);

            println!("[{}] Deleting remote zip (if any)", collection_name);
            self.arc_sandbox.lock().unwrap().delete_remote_collection(remote_zip_path.as_str());

            println!("[{}] Sending zip to remote", collection_name);
            self.arc_sandbox.lock().unwrap().send_collection(collection_zip, remote_zip_path.as_str());

            println!("[{}] Deleting remote folder", collection_name);
            self.arc_sandbox.lock().unwrap().delete_remote_collection(remote_folder_path.as_str());

            println!("[{}] Unzipping remote zip", collection_name);
            self.arc_sandbox.lock().unwrap().unzip_remote_zip(remote_zip_path.as_str());

            println!("[{}] Deleting remote zip", collection_name);
            self.arc_sandbox.lock().unwrap().delete_remote_collection(remote_zip_path.as_str());
        }
    }
}