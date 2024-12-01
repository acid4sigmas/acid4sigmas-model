use anyhow::anyhow;
use colored::*;
use once_cell::sync::OnceCell;
use std::env;
use std::fs::read_to_string;
use toml::Value;

// i like statics. dont blame me..

pub static SECRET_KEY: OnceCell<String> = OnceCell::new();
pub static DB_NAME: OnceCell<String> = OnceCell::new();
pub static DB_PW: OnceCell<String> = OnceCell::new();
pub static DB_PORT: OnceCell<String> = OnceCell::new();
pub static NO_REPLY_EMAIL: OnceCell<String> = OnceCell::new();
pub static SMTP_USERNAME: OnceCell<String> = OnceCell::new();
pub static SMTP_PASSWORD: OnceCell<String> = OnceCell::new();
pub static SMTP_RELAY: OnceCell<String> = OnceCell::new();
pub static DB_WS_URL: OnceCell<String> = OnceCell::new();
pub static REPO: OnceCell<String> = OnceCell::new();
pub static OWNER: OnceCell<String> = OnceCell::new();

fn load_secret(data: &Value, key: &str) -> Option<String> {
    data.get(key)
        .and_then(|val| val.as_str().map(|s| s.to_string()))
}
pub fn init_secrets(path: &str) -> anyhow::Result<()> {
    let contents = read_to_string(path).map_err(|e| {
        let current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        anyhow!(
            "Failed to read Secrets.toml from path '{}'. Current working directory: '{}'. Error: {}",
            path,
            current_dir,
            e
        )
    })?;

    // Attempt to parse the file
    let data: Value = contents
        .parse()
        .map_err(|e| anyhow!("Failed to parse Secrets.toml: {}", e))?;

    println!(
        "{}",
        "[info] missing keys are treated as empty strings.".blue()
    );

    macro_rules! set_or_warn {
        ($cell:expr, $key:expr) => {
            if let Some(value) = load_secret(&data, $key) {
                $cell
                    .set(value)
                    .expect(concat!($key, " is already initialized"));
            } else {
                println!("{}", format!("[warning] no {} found", $key).yellow());
                $cell
                    .set(String::new())
                    .expect(concat!("Failed to set empty ", $key));
            }
        };
    }

    set_or_warn!(SECRET_KEY, "SECRET_KEY");
    set_or_warn!(DB_NAME, "DB_NAME");
    set_or_warn!(DB_PW, "DB_PW");
    set_or_warn!(DB_PORT, "DB_PORT");
    set_or_warn!(NO_REPLY_EMAIL, "NO_REPLY_EMAIL");
    set_or_warn!(SMTP_USERNAME, "SMTP_USERNAME");
    set_or_warn!(SMTP_PASSWORD, "SMTP_PASSWORD");
    set_or_warn!(SMTP_RELAY, "SMTP_RELAY");
    set_or_warn!(DB_WS_URL, "DB_WS_URL");
    set_or_warn!(OWNER, "OWNER");

    if let Some(repos) = load_repos(&data) {
        REPO.set(repos).expect("REPO is already initialized");
    } else {
        println!("{}", "[warning] no REPO found".yellow());
        REPO.set(String::new()).expect("Failed to set empty REPO");
    }

    Ok(())
}

fn load_repos(data: &Value) -> Option<String> {
    if let Some(repo_array) = data.get("REPO").and_then(|v| v.as_array()) {
        Some(
            repo_array
                .iter()
                .filter_map(|val| val.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
                .join(","),
        )
    } else {
        None
    }
}
