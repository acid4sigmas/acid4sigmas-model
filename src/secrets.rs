use colored::*;
use once_cell::sync::OnceCell;
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
pub fn init_secrets(path: &str) {
    let contents = read_to_string(path).expect("failed to read Secrets.toml");
    let data: Value = contents.parse().expect("failed to parse Secrets.toml");

    println!(
        "{}",
        "[info] missing keys are treated as empty strings.".blue()
    );

    if let Some(secret) = load_secret(&data, "SECRET_KEY") {
        SECRET_KEY
            .set(secret)
            .expect("SECRET_KEY is already initialized");
    } else {
        println!("{}", "[warning] no SECRET_KEY found".yellow());
        SECRET_KEY
            .set(String::new())
            .expect("Failed to set empty SECRET_KEY");
    }

    if let Some(db_name) = load_secret(&data, "DB_NAME") {
        DB_NAME
            .set(db_name)
            .expect("DB_NAME is already initialized");
    } else {
        println!("{}", "[warning] no DB_NAME found".yellow());
        DB_NAME
            .set(String::new())
            .expect("Failed to set empty DB_NAME");
    }

    if let Some(db_pw) = load_secret(&data, "DB_PW") {
        DB_PW.set(db_pw).expect("DB_PW is already initialized");
    } else {
        println!("{}", "[warning] no DB_PW found".yellow());
        DB_PW.set(String::new()).expect("Failed to set empty DB_PW");
    }

    if let Some(db_port) = load_secret(&data, "DB_PORT") {
        DB_PORT
            .set(db_port)
            .expect("DB_PORT is already initialized");
    } else {
        println!("{}", "[warning] no DB_PORT found".yellow());
        DB_PORT
            .set(String::new())
            .expect("Failed to set empty DB_PORT");
    }

    if let Some(no_reply_email) = load_secret(&data, "NO_REPLY_EMAIL") {
        NO_REPLY_EMAIL
            .set(no_reply_email)
            .expect("NO_REPLY_EMAIL is already initialized");
    } else {
        println!("{}", "[warning] no NO_REPLY_EMAIL found".yellow());
        NO_REPLY_EMAIL
            .set(String::new())
            .expect("Failed to set empty NO_REPLY_EMAIL");
    }

    if let Some(smtp_username) = load_secret(&data, "SMTP_USERNAME") {
        SMTP_USERNAME
            .set(smtp_username)
            .expect("SMTP_USERNAME is already initialized");
    } else {
        println!("{}", "[warning] no SMTP_USERNAME found".yellow());
        SMTP_USERNAME
            .set(String::new())
            .expect("Failed to set empty SMTP_USERNAME");
    }

    if let Some(smtp_password) = load_secret(&data, "SMTP_PASSWORD") {
        SMTP_PASSWORD
            .set(smtp_password)
            .expect("SMTP_PASSWORD is already initialized");
    } else {
        println!("{}", "[warning] no SMTP_PASSWORD found".yellow());
        SMTP_PASSWORD
            .set(String::new())
            .expect("Failed to set empty SMTP_PASSWORD");
    }

    if let Some(smtp_relay) = load_secret(&data, "SMTP_RELAY") {
        SMTP_RELAY
            .set(smtp_relay)
            .expect("SMTP_RELAY is already initialized");
    } else {
        println!("{}", "[warning] no SMTP_RELAY found".yellow());
        SMTP_RELAY
            .set(String::new())
            .expect("Failed to set empty SMTP_RELAY");
    }

    if let Some(db_ws_url) = load_secret(&data, "DB_WS_URL") {
        DB_WS_URL
            .set(db_ws_url)
            .expect("DB_WS_URL is already initialized");
    } else {
        println!("{}", "[warning] no DB_WS_URL found".yellow());
        DB_WS_URL
            .set(String::new())
            .expect("Failed to set empty DB_WS_URL");
    }

    if let Some(repos) = load_repos(&data) {
        REPO.set(repos).expect("REPO is already initialized");
    } else {
        println!("{}", "[warning] no REPO found".yellow());
        REPO.set(String::new()).expect("Failed to set empty REPO");
    }

    if let Some(owner) = load_secret(&data, "OWNER") {
        OWNER.set(owner).expect("OWNER is already initialized");
    } else {
        println!("{}", "[warning] no OWNER found".yellow());
        OWNER.set(String::new()).expect("Failed to set empty OWNER");
    }
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
