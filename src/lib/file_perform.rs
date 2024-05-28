use super::file_handel::CommandFS;
use std::path::Path;
use tokio::fs;

impl<'a> CommandFS<'a> {
    /**
    # Example
    ```rust
    let mut new_command = CommandFS::new("/");
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
    *# Example
    ```rust
    let mut new_command = CommandFS::new("/");
    new_command.read_data
    ```
    *
    * */
    pub async fn read_data(&mut self, from_file: &str) -> Vec<u8> {
        match fs::read(self.whereami().to_owned() + from_file).await {
            Ok(file) => file,
            Err(error) => {
                self.err_msg = error.to_string();
                vec![]
            }
        }
    }
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
}
