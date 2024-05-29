use command_fs::file_handel::CommandFS;

#[tokio::main]
async fn main() {
    let mut command = CommandFS::new("/");
    let home_dir = command.know_home_dir();
    command.change_dir(home_dir.as_str());

    let search = ".bash";

    // Print every file content
    // Method 1
    for file in command.query_file(search, true) {
        println!("{file}");
        println!(
            "{}",
            String::from_utf8(command.read_data(file.as_str()).await).unwrap()
        );
        println!("{}", command.err_msg);
    }

    // Method 2
    for file in command.query_file(search, true) {
        println!("{file}");
        println!(
            "{}",
            String::from_utf8(&mut command >> file.as_str()).unwrap()
        );
        println!("{}", command.err_msg);
    }
}
