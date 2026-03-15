use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WriteFileArgs {
    file_path: String,
    content: String,
}

pub fn write_file(args: &str) -> String {
    let args: WriteFileArgs = match serde_json::from_str(args) {
        Ok(parsed) => parsed,
        Err(e) => return format!("Failed to parse arguments: {}", e),
    };
    let file_path = args.file_path;
    let content = args.content;

    match std::fs::write(file_path, content) {
        Ok(()) => "File written successfully".to_string(),
        Err(e) => format!("Failed to write file: {}", e),
    }
}
