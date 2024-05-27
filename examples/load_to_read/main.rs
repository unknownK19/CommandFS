use command_fs::file_handel::CommandFS;

fn main() {
    let mut new_command = CommandFS::new("/home/anurag/");
    println!("Accuracy True\n");
    for dir in new_command.query_file_dir("Meta", true) {
        println!("{dir}")
    }
    println!("{}", new_command.err_msg);
    println!("\nAccuracy False\n");
    for dir in new_command.query_file_dir("esp", false) {
        println!("{dir}")
    }
}
