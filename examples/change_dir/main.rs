use command_fs::file_handel::CommandFS;

fn main() {
    // Method 1
    let mut command = CommandFS::new("/");
    command.change_dir("home");
    println!("{}", command.whereami());
    println!("{}", command.err_msg);

    // Method 2
    let mut command = CommandFS::new("/");
    command >>= "home";
    println!("{}", command.whereami());
    println!("{}", command.err_msg);
}
