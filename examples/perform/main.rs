use command_fs::file_handel::CommandFS;

#[tokio::main]
async fn main() {
    let mut new_command = CommandFS::new("/home/anurag/");
    let home_dir = new_command.know_home_dir();
    new_command.change_dir(home_dir.as_str());
    new_command
        .write_data(b"Hello World".to_vec(), "Metadata.txt")
        .await;
    println!("{}", new_command.whereami()); // Current File or Directory
    println!("{}", new_command.err_msg) // print error message (optional) panicless
}
