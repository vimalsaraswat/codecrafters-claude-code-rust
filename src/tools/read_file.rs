use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ReadFileArgs {
    file_path: String,
}

pub fn read_file(args: &str) -> String {
    let args: ReadFileArgs = match serde_json::from_str(args) {
        Ok(parsed) => parsed,
        Err(e) => return format!("Failed to parse arguments: {}", e),
    };
    let file_path = args.file_path;

    match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => format!("Failed to read file: {}", e),
    }
}
