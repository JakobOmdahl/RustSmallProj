use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

fn main() {
    create_file_if_not_exists();
    print!("\x1B[2J\x1B[1;1H");
    loop {
        let mut input = String::new();
        println!("Enter what you want to do:");
        println!("new/existing/view/exit");
        std::io::stdin().read_line(&mut input).unwrap();

        let option = match input.trim() {
            "new" => Options::New,
            "existing" => Options::Existing,
            "view" => Options::View,
            "exit" => Options::Exit,
            _ => Options::Error,
        };
        match option {
            Options::New => {
                let mut file = OpenOptions::new().append(true).open("notes.txt").unwrap();
                let mut title = String::new();
                println!("Enter title:");
                std::io::stdin().read_line(&mut title).unwrap();
                let mut content = String::new();
                println!("Enter content:");
                std::io::stdin().read_line(&mut content).unwrap();
                let note = Note::new(title.trim().to_string(), content.trim().to_string());
                file.write_all(format!("{}|{}\n", note.title, note.content).as_bytes())
                    .unwrap();
                print!("\x1B[2J\x1B[1;1H");
            }
            Options::Existing => {
                let mut updated_lines: Vec<String> = Vec::new();
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("notes.txt")
                    .unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let lines = contents.lines();
                let mut loop_rule = 0;
                let mut input2 = String::new();
                for line in lines {
                    let parts: Vec<&str> = line.splitn(2, '|').collect();
                    if parts.len() == 2 {
                        let mut title = parts[0].trim().to_string();
                        let mut content = parts[1].trim().to_string();
                        println!("The current note is this:\n\n");
                        println!("Title: {}\n{}", title, content);

                        if loop_rule == 0 {
                            println!("\n");
                            println!("What would you like to do?:");
                            println!("edit/delete/next/exit");
                            input2.clear();
                            std::io::stdin().read_line(&mut input2).unwrap();
                        } else {
                            input2.clear();
                            input2 = "next".to_string();
                        }

                        let existing_option = match input2.trim() {
                            "edit" => ExistingOptions::Edit,
                            "delete" => ExistingOptions::Delete,
                            "next" => ExistingOptions::Next,
                            _ => ExistingOptions::Exit,
                        };
                        match existing_option {
                            ExistingOptions::Edit => {
                                println!("");
                                println!("Enter new title or leave blank to keep the same:");
                                let mut new_title = String::new();
                                std::io::stdin().read_line(&mut new_title).unwrap();
                                let new_title = new_title.trim().to_string();
                                if !new_title.is_empty() {
                                    title = new_title
                                }
                                println!("Enter new content:");
                                let mut new_content = String::new();
                                std::io::stdin().read_line(&mut new_content).unwrap();
                                if !new_content.is_empty() {
                                    content = new_content.trim().to_string();
                                }
                                updated_lines.push(format!("{}|{}", title, content));
                                print!("\x1B[2J\x1B[1;1H");
                            }
                            ExistingOptions::Delete => {}
                            ExistingOptions::Next => {
                                updated_lines.push(line.to_string());
                                print!("\x1B[2J\x1B[1;1H");
                            }
                            ExistingOptions::Exit => {
                                updated_lines.push(line.to_string());
                                loop_rule = 1;
                            }
                        }
                    }
                    print!("\x1B[2J\x1B[1;1H");
                }
                let updated_content = updated_lines.join("\n") + "\n";
                file.set_len(0).unwrap();
                file.seek(SeekFrom::Start(0)).unwrap();
                file.write_all(updated_content.as_bytes()).unwrap();
            }
            Options::View => {
                let mut file = OpenOptions::new().read(true).open("notes.txt").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let lines = contents.lines();
                for line in lines {
                    let parts: Vec<&str> = line.splitn(2, '|').collect();
                    if parts.len() == 2 {
                        let title = parts[0].trim().to_string();
                        let content = parts[1].trim().to_string();
                        println!("");
                        println!("Title: {}", title);
                        println!("Content: {}", content);
                        println!("");
                    } else {
                        println!("{}", line);
                        break;
                    }
                }
                let mut continue_program = String::new();
                std::io::stdin().read_line(&mut continue_program).unwrap();
                print!("\x1B[2J\x1B[1;1H");
            }
            Options::Error => {
                println!("Invalid input, please write one of the options given:");
            }
            Options::Exit => {
                println!("Exiting Program...");
                break;
            }
        }
    }
}
enum Options {
    New,
    Existing,
    View,
    Error,
    Exit,
}
enum ExistingOptions {
    Edit,
    Delete,
    Next,
    Exit,
}

struct Note {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

fn create_file_if_not_exists() {
    let path = std::path::Path::new("notes.txt");
    if !path.exists() {
        let _ = std::fs::File::create(path);
    }
}
