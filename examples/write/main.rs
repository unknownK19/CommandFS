use command_fs::file_handel::CommandFS;

#[tokio::main]
async fn main() {
    // Method 1 For Async
    let mut command = CommandFS::new("/");
    let home_dir = command.know_home_dir();
    command.change_dir(home_dir.as_str());
    command
        .write_data(b"It's November".to_vec(), "metadata.txt")
        .await;
    println!(
        "{}",
        String::from_utf8(command.read_data("metadata.txt").await).unwrap()
    ); // Print Content Asynchronously
    println!("{}", command.err_msg);

    // Method 2 For Sync
    let mut command = CommandFS::new("/");
    command >>= home_dir.as_str();
    command += ("metadata.txt", b"It's January Now");
    println!(
        "{}",
        String::from_utf8(&mut command >> "metadata.txt").unwrap()
    ); // Print Content Normally
}
