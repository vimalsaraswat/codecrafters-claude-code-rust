use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RunBashCommandArgs {
    command: String,
}

pub fn run_bash_command(args: &str) -> String {
    let args: RunBashCommandArgs = match serde_json::from_str(args) {
        Ok(parsed) => parsed,
        Err(e) => return format!("Failed to parse arguments: {}", e),
    };
    let command = args.command;

    match std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("stdout: {}\nstderr: {}", stdout, stderr)
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}
