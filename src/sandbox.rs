use crate::config::ConfigData;

use rustydav::client;
use rustydav::prelude::*;

#[derive(Debug)]
pub struct Status {
    status: Result<u16, u16>,
    message: String,
}

pub struct Sandbox {
    webdav: client::Client,
    url: String,
    code_version: String,
}

impl Status {
    pub fn success(status_code: u16, message: &str) -> Self {
        Status {
            status: Ok(status_code),
            message: message.to_owned(),
        }
    }

    pub fn error(status_code: u16, message: &str) -> Self {
        Status {
            status: Err(status_code),
            message: message.to_owned(),
        }
    }
}

impl Sandbox {
    pub fn init(config: &Box<dyn ConfigData>, url: &str) -> Self {
        Sandbox {
            webdav: client::Client::init(config.get_username(), config.get_password()),
            url: url.to_owned(),
            code_version: config.get_code_version().to_owned(),
        }
    }

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }

    fn path_with_version(&self, path: &str) -> String {
        format!("{}/{}{}", self.url, self.code_version, path)
    }

    fn parse_response_status(&self, result: &mut Response, message: &str) -> Result<Status, String> {
        let status_code: u16 = result.status().as_u16();
        match status_code {
            200 | 201 | 204 => Ok(Status::success(status_code, "success")),
            401 => Ok(Status {
                status: Err(status_code),
                message: format!("Unauthorized call! {}", message),
            }),
            403 => Ok(Status {
                status: Err(status_code),
                message: format!("Forbidden!, You don't have permission to {} !", message),
            }),
            404 => Ok(Status {
                status: Err(status_code),
                message: "Resource no longer exist".to_owned(),
            }),
            409 => Ok(Status {
                status: Err(status_code),
                message: "Conflict, A collection cannot be made at the Request-URI until one or more intermediate collections have been created.".to_owned(),
            }),
            502 => Ok(Status {
                status: Err(status_code),
                message: "Bad Gateway, Server refuses to accept the resource.".to_owned(),
            }),
            507 => Ok(Status {
                status: Err(status_code),
                message: "Insufficient Storage, The destination resource does not have sufficient space to record the state of the resource after the execution of this method.".to_owned(),
            }),
            _ => Err(format!("Request failed\nUrl: {}\nStatus: {}\nAddress: {}", result.url().as_str(), result.status(), result.remote_addr().unwrap())),
        }
    }

    /// Gets and parses .version file from DW Sandbox to get the currently used code version
    pub fn get_active_codeversion(&self) -> Result<String, String> {
        match self.webdav.get(self.path("/.version").as_str()) {
            Ok(mut result) => {
                match self.parse_response_status(&mut result, "wrong username or password!") {
                    Ok(response) => {
                        match response.status {
                            Ok(_) => Ok(
                                result.text().unwrap()
                                .split("\n")
                                .nth(6).unwrap()
                                .split("/")
                                .nth(0).unwrap()
                                .to_owned()
                            ),
                            Err(status_code) => Err(format!("Error checking current active version: {0}, Status: {1}", response.message, status_code)),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err(error) => Err(format!("{}", error))
        }
    }

    /// Sends any type of file ( .txt, .json, .zip ...) to Sandbox
    /// collection should be any type that reqwest accepts as a Body
    /// remote_path must be relative path on Sandbox to the current active code version including the file name and extension
    pub fn send_collection<B: Into<Body>>(&self, collection: B, remote_path: &str) -> Result<Status, String> {
        match self.webdav.put(collection, self.path_with_version(remote_path).as_ref()) {
            Ok(mut result) => self.parse_response_status(&mut result, "send file"),
            Err(e) => Err(format!("Error sending file to Sandbox: {}\nRemote path: {}", e, remote_path)),
        }
    }

    pub fn unzip_remote_zip(&self, path: &str) -> Result<Status, String> {
        match self.webdav.unzip(self.path_with_version(path).as_str()) {
            Ok(response) => Ok(Status {
                status: Ok(response.status().as_u16()),
                message: "success".to_owned(),
            }),
            Err(e) => Err(format!("Error unzipping the cartridge: {0}", e)),
        }
    }

    /// Deletes collections, files or folders from Webdav server
    /// If the folder contains other folders or files they will be also deleted
    pub fn delete_remote_collection(&self, path: &str) -> Result<Status, String> {
        match self.webdav.delete(self.path_with_version(path).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "delete remote file"),
            Err(e) => Err(format!("Error deleting remote file: {}\nRemote path: {}", e, path)),
        }
    }

    pub fn create_dir(&self, path: &str) -> Result<Status, String> {
        match self.webdav.mkcol(self.path_with_version(path).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "create folder"),
            Err(e) => Err(format!("Error creating directory: {0}", e)),
        }
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<Status, String> {
        match self.webdav.mv(self.path_with_version(from).as_str(), self.path_with_version(to).as_str()) {
            Ok(mut result) => self.parse_response_status(&mut result, "rename collection"),
            Err(e) => Err(format!("Error renaming collection: {}!\nFrom: {}\nTo: {}", e, from, to)),
        }
    }
}