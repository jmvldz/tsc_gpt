use regex::Regex;
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::io::{self, Read};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tsc_output = run_tsc()?;

    let errors = parse_tsc_errors(&tsc_output);

    println!("Parsed Errors:");
    for (index, error) in errors.iter().enumerate() {
        println!(
            "Error {}: File: {}, Message: {}",
            index + 1,
            error.file_path,
            error.error_message
        );
    }
    println!();

    for error in errors {
        let file_content = fs::read_to_string(&error.file_path)?;
        let summary = get_error_summary(&error, &file_content)?;

        println!("Error Details:");
        println!("  File: {}", error.file_path);
        println!("  Error: {}", error.error_message);
        println!("  Summary:");
        for line in summary.lines() {
            println!("    {}", line);
        }
        println!();
    }

    Ok(())
}

fn run_tsc() -> Result<String, Box<dyn std::error::Error>> {
    println!("Current directory: {:?}", std::env::current_dir()?);

    let output = Command::new("tsc").arg("--version").output()?;

    if output.status.success() {
        println!(
            "TypeScript version: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        println!("Failed to get TypeScript version");
    }

    let output = Command::new("tsc").arg("--noEmit").output()?;

    println!("tsc exit status: {:?}", output.status);
    println!("tsc stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("tsc stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        Ok(String::from_utf8(output.stderr)?)
    } else {
        println!("No TypeScript errors found or tsc didn't run correctly.");
        Ok(String::new())
    }
}

struct TscError {
    file_path: String,
    error_message: String,
}

fn parse_tsc_errors(input: &str) -> Vec<TscError> {
    let file_regex = Regex::new(r"(.*?)\((\d+),(\d+)\): error TS(\d+): (.*)").unwrap();
    let mut errors = Vec::new();

    for line in input.lines() {
        if let Some(captures) = file_regex.captures(line) {
            errors.push(TscError {
                file_path: captures[1].to_string(),
                error_message: format!(
                    "TS{}: {}",
                    captures.get(4).map_or("", |m| m.as_str()),
                    captures.get(5).map_or("", |m| m.as_str())
                ),
            });
        }
    }

    errors
}

fn get_error_summary(
    error: &TscError,
    file_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY")?;

    let prompt = format!(
        "Summarize the following TypeScript error in a short, readable manner:\n\nFile: {}\nError: {}\n\nRelevant file content:\n{}",
        error.file_path, error.error_message, file_content
    );

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant that summarizes TypeScript errors."},
                {"role": "user", "content": prompt}
            ]
        }))
        .send()?;

    let result: serde_json::Value = response.json()?;
    let summary = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Failed to get summary")
        .to_string();

    Ok(summary)
}
