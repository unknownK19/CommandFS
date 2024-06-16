use super::file_handel::CommandFS;

impl<'a> CommandFS<'a> {
    pub fn whereami(&mut self) -> &str {
        match self.dir.to_str() {
            Some(path) => path,
            None => {
                self.err_msg = "Unable to know Path".to_string();
                ""
            }
        }
    }
    pub fn know_home_dir(&mut self) -> String {
        match homedir::get_my_home() {
            Ok(home_option) => match home_option {
                Some(home) => {
                    let home_bind = home.clone();
                    match home_bind.to_str() {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    }
                }
                None => {
                    self.err_msg = "Somethings Wrong".to_string();
                    "".to_string()
                }
            },
            Err(error) => {
                self.err_msg = error.to_string();
                "".to_string()
            }
        }
    }
    pub fn dir_list(&mut self) -> Vec<String> {
        match self.dir.read_dir() {
            Ok(read_dir) => {
                let mut output = vec![];
                for dir in read_dir {
                    match dir {
                        Ok(dir_entry) => {
                            if dir_entry.path().is_dir() {
                                let len_path = match self.dir.to_str() {
                                    Some(str) => str.len(),
                                    None => 0,
                                };
                                output.push(
                                    dir_entry.path().display().to_string().as_str()[len_path..]
                                        .to_string(),
                                )
                            }
                        }
                        Err(error) => self.err_msg = error.to_string(),
                    }
                }
                return output;
            }
            Err(error) => {
                self.err_msg = error.to_string();
                vec![]
            }
        }
    }

    pub fn file_list(&mut self) -> Vec<String> {
        if !self.dir.is_dir() {
            self.err_msg = String::from(
                "!WARNING! Seems like Given path is file please set path to Directory on given",
            );
            vec![]
        } else {
            match self.dir.read_dir() {
                Ok(read_dir) => {
                    let mut output = vec![];
                    for dir in read_dir {
                        match dir {
                            Ok(dir_entry) => {
                                if !dir_entry.path().is_dir() {
                                    let len_path = match self.dir.to_str() {
                                        Some(str) => str.len(),
                                        None => 0,
                                    };
                                    output.push(
                                        dir_entry.path().display().to_string().as_str()[len_path..]
                                            .to_string(),
                                    )
                                }
                            }
                            Err(error) => self.err_msg = error.to_string(),
                        }
                    }
                    return output;
                }
                Err(error) => {
                    self.err_msg = error.to_string();
                    vec![]
                }
            }
        }
    }
    pub fn file_dir_list(&mut self) -> Vec<String> {
        if !self.dir.is_dir() {
            self.err_msg = String::from(
                "!WARNING! Seems like Given path is file please set path to Directory on given",
            );
            vec![]
        } else {
            match self.dir.read_dir() {
                Ok(read_dir) => {
                    let mut output = vec![];
                    for dir in read_dir {
                        match dir {
                            Ok(dir_entry) => {
                                let len_path = match self.dir.to_str() {
                                    Some(str) => str.len(),
                                    None => 0,
                                };
                                output.push(
                                    dir_entry.path().display().to_string().as_str()[len_path..]
                                        .to_string(),
                                )
                            }
                            Err(error) => self.err_msg = error.to_string(),
                        }
                    }
                    return output;
                }
                Err(error) => {
                    self.err_msg = error.to_string();
                    vec![]
                }
            }
        }
    }
    pub fn query_dir(&mut self, query: &'a str, accurate: bool) -> Vec<String> {
        let mut output = vec![];
        let mut listed_dir = self.dir_list();
        while listed_dir.len() != 0 {
            if accurate {
                match listed_dir.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_dir.pop()
                        } else {
                            listed_dir.pop()
                        };
                    }
                    None => break,
                }
            } else {
                let char_q: Vec<char> = query.chars().collect();
                match listed_dir.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_dir.pop()
                        } else {
                            let mut allow = false;
                            for each_char in char_q {
                                if x.matches(each_char).collect::<Vec<&str>>().len() != 0 {
                                    allow = true;
                                } else {
                                    allow = false;
                                    break;
                                }
                            }
                            if allow {
                                output.push(x.to_string());
                            }
                            listed_dir.pop()
                        };
                    }
                    None => break,
                }
            }
        }
        output
    }
    pub fn query_file(&mut self, query: &'a str, accurate: bool) -> Vec<String> {
        let mut output = vec![];
        let mut listed_file = self.file_list();
        while listed_file.len() != 0 {
            if accurate {
                match listed_file.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_file.pop()
                        } else {
                            listed_file.pop()
                        };
                    }
                    None => break,
                }
            } else {
                let char_q: Vec<char> = query.chars().collect();
                match listed_file.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_file.pop()
                        } else {
                            let mut allow = false;
                            for each_char in char_q {
                                if x.matches(each_char).collect::<Vec<&str>>().len() != 0 {
                                    allow = true;
                                } else {
                                    allow = false;
                                    break;
                                }
                            }
                            if allow {
                                output.push(x.to_string());
                            }
                            listed_file.pop()
                        };
                    }
                    None => break,
                }
            }
        }
        output
    }
    pub fn query_file_dir(&mut self, query: &'a str, accurate: bool) -> Vec<String> {
        let mut output = vec![];
        let mut listed_file_dir = self.file_dir_list();
        while listed_file_dir.len() != 0 {
            if accurate {
                match listed_file_dir.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_file_dir.pop()
                        } else {
                            listed_file_dir.pop()
                        };
                    }
                    None => break,
                }
            } else {
                let char_q: Vec<char> = query.chars().collect();
                match listed_file_dir.last() {
                    Some(x) => {
                        if x.matches(query).collect::<Vec<&str>>().len() != 0 {
                            output.push(x.to_string());
                            listed_file_dir.pop()
                        } else {
                            let mut allow = false;
                            for each_char in char_q {
                                if x.matches(each_char).collect::<Vec<&str>>().len() != 0 {
                                    allow = true;
                                } else {
                                    allow = false;
                                    break;
                                }
                            }
                            if allow {
                                output.push(x.to_string());
                            }
                            listed_file_dir.pop()
                        };
                    }
                    None => break,
                }
            }
        }
        output
    }
}
