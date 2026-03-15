use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use codecrafters_claude_code::{
    message::Message,
    tools::{read_file::read_file, run_bash_command::run_bash_command, write_file::write_file},
};
use serde::Deserialize;
use serde_json::json;
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    // finish_reason: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let mut messages: Vec<Message> = Vec::new();

    messages.push(Message::new_user(Some(args.prompt.clone())));

    loop {
        let response: Response = client
            .chat()
            .create_byot(json!({
                "messages": serde_json::to_value(&messages)?,
                "model": "anthropic/claude-haiku-4.5",
                "tools": [
                    {
                      "type": "function",
                      "function": {
                        "name": "read_file",
                        "description": "Read and return the contents of a file",
                        "parameters": {
                          "type": "object",
                          "properties": {
                            "file_path": {
                              "type": "string",
                              "description": "The path to the file to read"
                            }
                          },
                          "required": ["file_path"]
                        }
                      }
                    },
                    {
                      "type": "function",
                      "function": {
                        "name": "write_file",
                        "description": "Write content to a file",
                        "parameters": {
                          "type": "object",
                          "required": ["file_path", "content"],
                          "properties": {
                            "file_path": {
                              "type": "string",
                              "description": "The path of the file to write to"
                            },
                            "content": {
                              "type": "string",
                              "description": "The content to write to the file"
                            }
                          }
                        }
                      }
                    },
                    {
                      "type": "function",
                      "function": {
                        "name": "run_bash_command",
                        "description": "Execute a shell command",
                        "parameters": {
                          "type": "object",
                          "required": ["command"],
                          "properties": {
                            "command": {
                              "type": "string",
                              "description": "The command to execute"
                            }
                          }
                        }
                      }
                    }
                ]
            }))
            .await?;

        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        let message = &response.choices[0].message;

        messages.push(message.clone());

        if let Some(tool_calls) = &message.tool_calls {
            for tool_call in tool_calls {
                let name = tool_call.function.name.as_str();
                let arguments = &tool_call.function.arguments;

                match name {
                    "read_file" => {
                        let file_content = read_file(arguments);
                        messages.push(Message::new_tool(tool_call.id.clone(), Some(file_content)));
                    }
                    "write_file" => {
                        let file_content = write_file(arguments);
                        messages.push(Message::new_tool(tool_call.id.clone(), Some(file_content)));
                    }
                    "run_bash_command" => {
                        let output = run_bash_command(arguments);
                        messages.push(Message::new_tool(tool_call.id.clone(), Some(output)));
                    }
                    _ => {
                        println!("Unknown tool");
                    }
                }
            }
        } else if let Some(content) = &message.content {
            println!("{}", content);
            break;
        }
    }

    Ok(())
}
