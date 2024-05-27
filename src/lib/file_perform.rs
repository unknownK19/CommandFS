use super::file_handel::CommandFS;
use super::file_handel::OperatingSystem;
use std::path::Path;
use tokio::fs;

impl<'a> CommandFS<'a> {
    #[cfg(target_os = "linux")]
    pub fn new(dir: &'a str) -> Self {
        Self {
            os: OperatingSystem::Linux,
            file_dir: Path::new(dir),
            // sample_dir: None,
            err_msg: "".to_string(),
        }
    }
    // Because it require some time to load large data
    pub async fn read_data(&mut self) -> Vec<u8> {
        match fs::read(self.file_dir).await {
            Ok(file) => file,
            Err(error) => {
                self.err_msg = error.to_string();
                vec![]
            }
        }
    }
    pub async fn write_data(&mut self, data: Vec<u8>, to_file: &'a str) {
        if self.file_dir.is_dir() {
            match fs::write(format!("{}{to_file}", self.whereami()), data).await {
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
            self.file_dir = Path::new(dir)
        } else {
            self.err_msg =
                "!ERROR! Change Dir failed cause given Path is file or not exit".to_string()
        }
    }
    // TODO: step back path like 'cd ..' command
    // pub fn step_back(&mut self) {
    //     let mut current_dir = self.whereami();
    //     current_dir
    //         .chars()
    //         .collect::<Vec<char>>()
    //         .iter()
    //         .map(|each| {});
    //     println!("{current_dir}")
    // }
}
