fn main() {
    create_file_if_not_exists();
    let mut notes: Vec<Note> = Vec::new();
    loop {
        let mut input = String::new();
        println!("Enter what you want to do:");
        println!("new/edit/delete");
        let option: String = std::io::stdin().read_line(&mut input).unwrap().to_string();
        enum Options {
            New,
            Edit,
            Delete,
        }
        let option = match option.trim() {
            "new" => Options::New,
            "edit" => Options::Edit,
            "delete" => Options::Delete,
            _ => Options::New,
        };
        match option {
            Options::New => {
                let mut title = String::new();
                println!("Enter title:");
                let _ = std::io::stdin().read_line(&mut title).unwrap();
                let mut content = String::new();
                println!("Enter content:");
                let _ = std::io::stdin().read_line(&mut content).unwrap();
                let note = Note::new(title.trim().to_string(), content.trim().to_string());
                notes.push(note);
            }
            _ => {}
        }
        save_notes(&notes);
    }
}

struct Note {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Self {
        if title.is_empty() {
            panic!("Title cannon be empty")
        }
        if content.is_empty() {
            panic!("Content cannot be empty")
        }
        Self { title, content }
    }
    fn edit(&mut self, title: String, content: String) {
        if title.is_empty() {
            panic!("Title cannot be empty")
        }
        if content.is_empty() {
            panic!("Content cannot be empty")
        }
        self.title = title;
        self.content = content;
    }
    fn delete(&mut self) {
        self.title = "".to_string();
        self.content = "".to_string();
    }
}

fn save_notes(notes: &[Note]) {
    let path = std::path::Path::new("notes.txt");
    let contents = notes.iter().fold(String::new(), |acc, note| {
        format!("{}{}{}", acc, note.title, note.content)
    });
    let _ = std::fs::write(path, contents);
}

fn create_file_if_not_exists() {
    let path = std::path::Path::new("notes.txt");
    if !path.exists() {
        let _ = std::fs::File::create(path);
    }
}
