use std::path::Path;

pub struct CommandFS<'a> {
    pub file_dir: &'a Path,
    // TODO: sample_dir: Option<Vec<Box<Path>>>,
    pub err_msg: String,
}
