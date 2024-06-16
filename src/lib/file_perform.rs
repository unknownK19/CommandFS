use super::file_handel::CommandFS;
use std::{
    borrow::BorrowMut,
    ops::{AddAssign, ShlAssign, Shr, ShrAssign, SubAssign},
    path::Path,
};
use tokio::fs;

impl<'a> CommandFS<'a> {
    /**
    # Example
    ```rust
    let mut command = CommandFS::new("/");
    ```
      It will create CommandFS struct which contain two Field. file_dir (File or Directory),
      err_msg (For Safety every error message is store in err_msg field)
     */
    pub fn new(dir: &'a str) -> Self {
        Self {
            dir: Path::new(dir),
            // sample_dir: None,
            err_msg: "".to_string(),
        }
    }
    // Because it require some time to load large data
    /**
    # Example
    ```rust
    let mut command = CommandFS::new("/home/username"); // For Unix System
    command.read_data(".bashrc"); // If you see Empty it means file Doesn't exist or something.
    println!("{}", command.err_msg); // It'll print error message without any panic
    ```
     */
    pub async fn read_data(&mut self, from_file: &str) -> Vec<u8> {
        match fs::read(self.whereami().to_owned() + "/" + from_file).await {
            Ok(file) => file,
            Err(error) => {
                self.err_msg = error.to_string();
                vec![]
            }
        }
    }
    /**
     # Example
     ```rust
    let mut command = CommandFS::new("/home/username"); // For Unix System
    command.read_data(".bashrc").await; // If you see Empty it means file Doesn't exist or something.
     ```
    */
    pub async fn write_data(&mut self, data: Vec<u8>, to_file: &'a str) {
        if self.dir.is_dir() {
            match fs::write(format!("{}/{to_file}", self.whereami()), data).await {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        } else {
            self.err_msg = String::from(
                "!WARNING! Seems like Given path is file please set path to Directory on given",
            );
        }
    }
    pub fn change_dir(&mut self, dir: &'a str) {
        if Path::new(dir).is_dir() {
            self.dir = Path::new(dir)
        } else {
            self.err_msg =
                "!ERROR! Change Dir failed cause given Path is file or not exist".to_string()
        }
    }
    pub fn step_back(&mut self, step: usize) {
        let mut current_step = step;
        let current_dir = self.whereami().to_owned();
        let mut splited: Vec<&str> = current_dir.split('/').filter(|s| !s.is_empty()).collect();
        let mut updated_path = "".to_string();
        {
            while current_step != 0 {
                match splited.pop() {
                    Some(_) => {}
                    None => {
                        self.err_msg = "!WARNING! it can't step back more".to_string();
                        self.dir = Path::new("/");
                        break;
                    }
                }
                current_step -= 1
            }
        }
        for each in &splited {
            updated_path.push_str(&("/".to_owned() + each));
        }
        if splited.len() == 0 {
            self.change_dir(Box::leak(Box::new("/")))
        } else {
            self.change_dir(Box::leak(Box::new(updated_path)))
        }
    }

    fn write_data_sync(&mut self, data: Vec<u8>, to_file: &'a str) {
        if self.dir.is_dir() {
            match std::fs::write(format!("{}/{to_file}", self.whereami()), data) {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        } else {
            self.err_msg = String::from(
                "!WARNING! Seems like Given path is file please set path to Directory on given",
            );
        }
    }
    pub async fn remove(&mut self, file: &str) {
        let target_path = self.whereami().to_owned() + "/" + file;
        if Path::new(target_path.as_str()).is_dir() {
            match fs::remove_dir(target_path.as_str()).await {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        } else {
            match fs::remove_file(target_path.as_str()).await {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        }
    }
    fn remove_sync(&mut self, path: &str) {
        let target_path = self.whereami().to_owned() + path;
        if Path::new(target_path.as_str()).is_dir() {
            match std::fs::remove_dir(target_path.as_str()) {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        } else {
            match std::fs::remove_file(target_path.as_str()) {
                Ok(_) => {}
                Err(error) => self.err_msg = error.to_string(),
            }
        }
    }
    fn read_data_sync(&mut self, from_file: &str) -> Vec<u8> {
        match std::fs::read(self.whereami().to_owned() + "/" + from_file) {
            Ok(file) => file,
            Err(error) => {
                self.err_msg = error.to_string();
                vec![]
            }
        }
    }
    fn create_dir(&mut self, dir_name: &'a str) {
        match std::fs::create_dir(format!("{}/{dir_name}", self.whereami())) {
            Ok(_) => {}
            Err(error) => self.err_msg = error.to_string(),
        }
    }
    pub fn rename(&mut self, from_file: &str, rename: &str) {
        let mut count = 0;
        let mut file_found = false;
        while self.file_list().len() != count - 1 {
            if self.file_list()[count] != from_file.to_string() {
                count += 1;
            } else {
                match std::fs::rename(
                    self.whereami().to_owned() + "/" + from_file,
                    self.whereami().to_owned() + "/" + rename,
                ) {
                    Ok(_) => {}
                    Err(error) => self.err_msg = error.to_string(),
                };
                file_found = true;
                break;
            }
        }
        if !file_found {
            self.err_msg = format!(
                "!NOT FOUND! from the currrent path, {} file is not available",
                from_file
            )
        }
    }
}

/**
# Example
    ```rust
let mut command = CommandFS::new("/home/username");
command += ("myfile.txt", b"Blah Blah Blah Blah");
    ```
      It will create Create and Write File for non sync task,
*/
impl<'a, const N: usize> AddAssign<(&'a str, &[u8; N])> for CommandFS<'a> {
    fn add_assign(&mut self, rhs: (&'a str, &[u8; N])) {
        self.write_data_sync(rhs.1.to_vec(), rhs.0);
    }
}

/**
# Example
    ```rust
let mut command = CommandFS::new("/home/username");
command += ("mydirectory");
    ```
      It will create Directory,
*/
impl<'a> AddAssign<&'a str> for CommandFS<'a> {
    fn add_assign(&mut self, rhs: &'a str) {
        self.create_dir(rhs)
    }
}

impl<'a> ShrAssign<&str> for CommandFS<'a> {
    fn shr_assign(&mut self, rhs: &str) {
        let updated_dir = self.whereami().to_owned() + rhs;
        self.borrow_mut()
            .change_dir(Box::leak(Box::new(updated_dir)));
    }
}

impl<'a> SubAssign<&str> for CommandFS<'a> {
    fn sub_assign(&mut self, rhs: &str) {
        self.remove_sync(rhs)
    }
}

impl<'a> ShlAssign<usize> for CommandFS<'a> {
    fn shl_assign(&mut self, rhs: usize) {
        self.step_back(rhs);
    }
}

impl<'a> Shr<&str> for &mut CommandFS<'a> {
    type Output = Vec<u8>;
    fn shr(self, rhs: &str) -> Self::Output {
        (&mut self.read_data_sync(rhs)).clone()
    }
}
