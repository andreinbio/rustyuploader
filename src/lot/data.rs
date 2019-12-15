use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct File {
    pub full_path: String,
    pub rel_path: String,
}

#[derive(Debug, Clone)]
pub struct Rename {
    pub current: String,
    pub new: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub upload: Option<Vec<File>>,
    pub rename: Option<Vec<Rename>>,
    pub remove: Option<Vec<String>>,
    base_path: String,
}

impl Data {
    pub fn new(base_path: &str) -> Self {
        Data {
            upload: None,
            rename: None,
            remove: None,
            base_path: base_path.to_owned()
        }
    }

    pub fn reset(&mut self) -> () {
        self.upload = None;
        self.rename = None;
        self.remove = None;
    }

    /// Files and Folders are created
    /// Push only files
    pub fn push_create(&mut self, path: PathBuf) -> () {
        self.push_write(path);
    }

    /// Push files with full and relative paths
    pub fn push_write(&mut self, path: PathBuf) -> () {
        let string_path: String = self.get_string_path(path);

        if self.is_file(&string_path) {
            self.push_file(&string_path);
        }
    }

    /// Check for files and folders
    /// If any folder is pased then remove any files that it contains
    pub fn push_remove(&mut self, path: PathBuf) -> () {
        let rel_path: String = self.get_relative_path(&self.get_string_path(path));

        self.remove = match self.remove.take() {
            Some(mut vec) => {
                vec.push(rel_path);
                Some(vec)
            },
            None => Some(vec![rel_path]),
        };
    }

    pub fn push_rename(&mut self, path: PathBuf, new_path: PathBuf) -> () {
        let current_name: String = self.get_relative_path(&self.get_string_path(path));
        let new_name: String = self.get_relative_path(&self.get_string_path(new_path));
        let rename_collection: Rename = Rename {
            current: current_name,
            new: new_name,
        };

        self.rename = match self.rename.take() {
            Some(mut vec) => {
                vec.push(rename_collection);
                Some(vec)
            },
            None => Some(vec![rename_collection]),
        }
    }

    /// Basic check if the path is a file or folder
    /// if the last part after the slahs contains any dot in it then it should be a file
    fn is_file(&self, path: &str) -> bool {
        path.split("/").last().unwrap_or("folder").split(".").count() > 1
    }

    /// Get the path as a string
    /// Also for windows replaces backslashes with slashes
    fn get_string_path(&self, path: PathBuf) -> String {
        path.to_str().take()
            .map(|s| s.to_string().replace("\\", "/"))
            .unwrap()
    }

    /// Transforms from absolute path into relative one
    fn get_relative_path(&self, path: &str) -> String {
        path.replace(self.base_path.as_str(), "")
    }

    fn push_file(&mut self, string_path: &str) -> () {
        let file_path = File {
            full_path: string_path.to_owned(),
            rel_path: self.get_relative_path(string_path)
        };

        self.upload = match self.upload.take() {
            Some(mut vec) => {
                vec.push(file_path);
                Some(vec)
            },
            None => Some(vec![file_path]),
        };
    }

    pub fn split_folders_files(&self) -> (Vec<String>, Vec<String>) {
        let mut folders = vec![];
        let mut files = vec![];

        self.remove.clone().unwrap_or(vec![]).iter()
            .for_each(|path| {
                if self.is_file(path) {
                    files.push(path.to_owned());
                } else {
                    folders.push(path.to_owned());
                }
            });

        (folders, files)
    }

    /// Update data
    pub fn update(&mut self) -> () {
        // update remove files and folders
        // if all the files from a folder are deleted prefer deleting the folder instead of individual files
        if self.remove.is_some() {
            let (mut folders, files) = self.split_folders_files();
            let mut filtered_folders = vec![];
            let mut final_remove;

            folders.sort_by(|folder_a, folder_b| {
                let count_a: usize = folder_a.split("/").count();
                let count_b: usize = folder_b.split("/").count();

                count_a.cmp(&count_b)
            });

            match folders.get(0) {
                Some(folder) => filtered_folders.push(folder.clone()),
                None => (),
            }

            for folder in folders.iter() {
                if !filtered_folders.contains(folder) {
                    let mut push = true;
                    for fill_folder in filtered_folders.clone().into_iter() {
                        if folder.contains(&fill_folder) || fill_folder.contains(folder) {
                            push = false;
                            break;
                        }
                    }

                    if push {
                        filtered_folders.push(folder.to_string());
                    }
                }
            }

            final_remove = filtered_folders.clone();

            for file in files.iter() {
                let mut push = true;
                for folder in filtered_folders.iter() {
                    if file.contains(folder) {
                        push = false;
                        break;
                    }
                }

                if push {
                    final_remove.push(file.to_owned());
                }
            }

            self.remove = Some(final_remove);
        }
    }

    /// Filter data
    ///
    pub fn filter(&mut self, paths: &Vec<String>, filter_type: &str) -> () {
        if self.upload.is_some() {
            self.upload = self.upload.take().map(|files| {
                files.into_iter()
                    .filter(|file| {
                        match filter_type {
                            "exclude" => paths.into_iter().all(|path| !file.rel_path.contains(path)),
                            _ => paths.into_iter().any(|path| file.rel_path.contains(path)),
                        }
                    }).collect::<Vec<File>>()
            });
        }

        if self.remove.is_some() {
            self.remove = self.remove.take().map(|collections| {
                collections.into_iter()
                    .filter(|remove| {
                        match filter_type {
                            "exclude" => paths.into_iter().all(|path| !remove.contains(path)),
                            _ => paths.into_iter().any(|path| remove.contains(path)),
                        }
                    }).collect::<Vec<String>>()
            });
        }

        if self.rename.is_some() {
            self.rename = self.rename.take().map(|collections| {
                collections.into_iter()
                    .filter(|rename| {
                        match filter_type {
                            "exclude" => paths.into_iter().all(|path| !rename.current.contains(path)),
                            _ => paths.into_iter().any(|path| rename.current.contains(path)),
                        }
                    }).collect::<Vec<Rename>>()
            });
        }
    }
}