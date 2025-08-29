use file_action::FileAction;

fn main() {
    println!("FileAction library example");

    let file_path = "example.txt";
    let content = FileAction::read(file_path);
    match content {
        Ok(text) => println!("File content:\n{}", text),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
