use super::json::Config;
use super::loader::*;
use super::time;

use rustydav::client;
use rustydav::prelude::*;

pub struct Sandbox {
    webdav: client::Client,
    url: String,
    code_version: String,
}

impl Sandbox {
    pub fn init(config: &Config) -> Self {
        Sandbox {
            webdav: client::Client::init(config.get_username().as_str(), config.get_password().as_str()),
            url: format!("https://{}/on/demandware.servlet/webdav/Sites/Cartridges", config.get_hostname()),
            code_version: config.get_code_version(),
        }
    }

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }

    fn path_with_version(&self, path: &str) -> String {
        format!("{}/{}{}", self.url, self.code_version, path)
    }

    fn parse_response_status(&self, result: &mut Response, message: &str) -> Result<(), String> {
        // dbg!(&result);
        match result.status().as_u16() {
            200 | 201 | 204 => Ok(()),
            401 => Err(format!("Unauthorized call! {}", message)),
            403 => Err(format!("Forbidden, You don't have permission to {} !", message)),
            404 => Err("Resource no longer exist".to_owned()),
            409 => Err("Conflict, A collection cannot be made at the Request-URI until one or more intermediate collections have been created.".to_owned()),
            502 => Err("Bad Gateway, Server refuses to accept the resource.".to_owned()),
            507 => Err("Insufficient Storage, The destination resource does not have sufficient space to record the state of the resource after the execution of this method.".to_owned()),
            _ => Err(format!("Request failed\nUrl: {}\nStatus: {}\nAddress: {}", result.url().as_str(), result.status(), result.remote_addr().unwrap())),
        }
    }

    /// Gets and parses .version file from DW Sandbox to get the currently used code version
    pub fn get_active_codeversion(&self) -> Result<String, String> {
        match self.webdav.get(self.path("/.version").as_str()) {
            Ok(mut result) => {
                match self.parse_response_status(&mut result, "wrong username or password!") {
                    Ok(()) => Ok(
                        result.text().unwrap()
                        .split("\n")
                        .nth(6).unwrap()
                        .split("/")
                        .nth(0).unwrap()
                        .to_owned()
                    ),
                    Err(message) => Err(message),
                }
            },
            Err(error) => Err(format!("{}", error))
        }
    }

    /// Sends any type of file ( .txt, .json, .zip ...) to Sandbox
    /// collection should be any type that reqwest accepts as a Body
    /// remote_path must be relative path on Sandbox to the current active code version including the file name and extension
    pub fn send_collection<B: Into<Body>>(&self, collection: B, remote_path: &str) -> Result<(), String> {
        match self.webdav.put(collection, self.path_with_version(remote_path).as_ref()) {
            Ok(mut result) => self.parse_response_status(&mut result, "send file"),
            Err(e) => Err(format!("Error sending file to Sandbox: {}\nRemote path: {}", e, remote_path)),
        }
    }

    pub fn unzip_remote_zip(&self, path: &str) -> Result<(), Error> {
        match self.webdav.unzip(self.path_with_version(path).as_str()) {
            Ok(result) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Deletes collections, files or folders from Webdav server
    /// If the folder contains other folders or files they will be also deleted
    pub fn delete_remote_collection(&self, path: &str) -> Result<(), String> {
        match self.webdav.delete(self.path_with_version(path).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "delete remote file"),
            Err(e) => Err(format!("Error deleting remote file: {}\nRemote path: {}", e, path)),
        }
    }

    pub fn create_dir(&self, path: &str) -> Result<(), String> {
        match self.webdav.mkcol(self.path_with_version(path).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "create folder"),
            Err(e) => Err("Error creating directory!".to_owned()),
        }
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<(), String> {
        match self.webdav.mv(self.path_with_version(from).as_str(), self.path_with_version(to).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "rename collection"),
            Err(e) => Err(format!("Error renaming collection: {}!\nFrom: {}\nTo: {}", e, from, to)),
        }
    }

    pub fn push_collection(&self, mut data: lot::Data) -> () {
        if data.rename.is_some() {
            let time = time::Time::new();
            for rename in data.rename.take().unwrap().iter() {
                let result = self.rename(rename.current.as_str(), rename.new.as_str());

                match result {
                    Ok(()) => println!("[R {}] from: {} to: {}", time.current().get_time(), rename.current, rename.new),
                    Err(message) => println!("[R {}] {}", time.current().get_time(), message),
                }
            }
        }

        if data.upload.is_some() {
            let time = time::Time::new();
            for file in data.upload.take().unwrap().iter() {
                let file_result = open_file(file.full_path.as_str());

                if file_result.is_err() {
                    println!("Unable to open file at path: {:?}", file.full_path.as_str());
                    continue;
                }

                let file_data = file_result.unwrap();
                let result = self.send_collection(file_data, file.rel_path.as_str());
                let current = time.current();

                match result {
                    Ok(()) => println!("[U {}] {}", current.get_time(), file.rel_path),
                    Err(message) => println!("{}", message),
                }
            }
        }

        if data.remove.is_some() {
            let time = time::Time::new();
            for path in data.remove.take().unwrap().iter() {
                let result = self.delete_remote_collection(path);
                let current = time.current();
                match result {
                    Ok(()) => println!("[D {}] {}", current.get_time(), path),
                    Err(message) => println!("{} at path: {}", message, path),
                }
            }
        }
    }
}