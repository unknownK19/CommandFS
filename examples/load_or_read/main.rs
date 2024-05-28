use command_fs::file_handel::CommandFS;

#[tokio::main]
async fn main() {
    let mut new_command = CommandFS::new("/");
    let home_dir = new_command.know_home_dir();
    new_command.change_dir(home_dir.as_str());

    println!("Accuracy True\n");
    for dir_file in new_command.query_file_dir("sh", true) {
        println!("{dir_file}")
    }

    println!("{}", new_command.err_msg); // Is there Any Error

    println!("\nAccuracy False\n");
    for dir_file in new_command.query_file_dir("sh", false) {
        println!("{dir_file}")
    }

    // Print every file content
    for file in new_command.query_file("sh", true) {
        println!("{file}");
        println!(
            "{}",
            String::from_utf8(new_command.read_data(file.as_str()).await).unwrap()
        );
        println!("{}", new_command.err_msg);
    }
}
