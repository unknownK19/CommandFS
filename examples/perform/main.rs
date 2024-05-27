use command_fs::file_handel::CommandFS;

#[tokio::main]
async fn main() {
    let mut new_command = CommandFS::new("/home/anurag/.zshrc");
    new_command
        .write_data(b"Hello World".to_vec(), "Metadata.txt")
        .await;
    // new_command.step_back();
    println!("{}", new_command.err_msg)
}
