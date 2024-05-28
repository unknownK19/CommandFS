use std::path::Path;

pub struct CommandFS<'a> {
    pub dir: &'a Path,
    // TODO: sample_dir: Option<Vec<Box<Path>>>,
    pub err_msg: String,
}
