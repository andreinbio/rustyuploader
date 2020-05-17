extern crate rustyuploader;

mod helpers;

use helpers::*;
use rustyuploader::upload;

const URL: &str = "https://www.webdavserver.com/User287e257";

#[test]
fn create_custom_json_file() {
    assert_eq!(create_json_file(), Ok(()));
}

#[test]
fn upload_cartridges() {
    let uploader = upload::Uploader::new_from_json(get_full_path("tests/config/dw.json").as_path(), Some(URL.to_owned()));

    // iterate over each cartridge and push to server
    for cartridge_name in uploader.get_cartridges() {
        // initialize cartridge to be pushed to server
        let mut cartridge_data = uploader.init_cartridge_upload(cartridge_name);

        assert_eq!(cartridge_data.is_some(), true, "Cartridge data couldn't be initialized!");

        // iterate over all actions and execute them
        for action in uploader.get_actions() {
            let result = uploader.push_cartridge(action, &mut cartridge_data);
            assert_eq!(result.is_ok(), true, "Error during action: {0:?} with result: {1:?}", action, result);
        }
    }
}