use notify::DebouncedEvent;

use super::data;

pub struct Collection {
    pub data: data::Data,
    cartridges: Vec<String>,
    ignore_list: Vec<String>,
}

impl Collection {
    pub fn init(base_path: &str, cartridges: Vec<String>, ignore_list: Vec<String>) -> Self {
        Collection {
            data: data::Data::new(base_path),
            cartridges: cartridges,
            ignore_list: ignore_list,
        }
    }

    pub fn parse_event(&mut self, event: DebouncedEvent) -> () {

        match event {
            DebouncedEvent::Create(path) => {
                // check only for files
                self.data.push_create(path);
            },
            DebouncedEvent::Write(path) => {
                // check only the files
                self.data.push_write(path);
            },
            DebouncedEvent::Remove(path) => {
                // check only the files and folders group by folder
                // check if folder exists and remove all the files that are part of this folder
                self.data.push_remove(path);
            },
            DebouncedEvent::Rename(path, new_path) => {
                // check only files and folders
                self.data.push_rename(path, new_path);
            },
            _ => (),
        }
    }

    // returns current available files and folders to be removed, added, update
    // sorts and updates the remove files and folders before returning them
    pub fn get_data(&mut self) -> data::Data {
        self.data.update();

        //filter only the files that are part of the "cartridges"
        self.data.filter(&self.cartridges, "include");

        if !self.ignore_list.is_empty() {
            self.data.filter(&self.ignore_list, "exclude");
        }

        let data: data::Data = self.data.clone();
        self.data.reset();

        data
    }
}