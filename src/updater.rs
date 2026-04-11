mod language;

use std::env;
use std::fs::{self, File};
use std::io::{copy, stdin};
use std::path::Path;
use reqwest::header::USER_AGENT;
use serde_json::Value;

fn main() {
    
    let lang = get_lang();
    let lang = lang.as_str();

    // 1. 원격 버전 정보 다운로드 시도
    let _ = download_version_info(lang);

    if let Some(new_url) = check_and_get_url(lang) {
        if let Err(e) = download_new(&new_url, lang) {
            eprintln!("{}: {}", language::get(lang, "FAIL_TO_DOWNLOAD"), e);
        }
    }
}


fn download_version_info(lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://githubyoon.github.io/Memo/version.txt";

    match reqwest::blocking::get(url) {
        Ok(mut response) => {
            let mut file = File::create("version.txt")?;
            copy(&mut response, &mut file)?;
            println!("{}", language::get(lang, "FETCHED_LATEST_VERSION"));
            Ok(())
        }
        Err(e) => {
            println!("{}: {}", language::get(lang, "FETCH_VERSION_FAILED_USE_LOCAL"), e);
            Ok(())
        }
    }
}

fn check_and_get_url(lang: &str) -> Option<String> {
    let content = fs::read_to_string("version.txt").ok()?;
    let remote_version = content.trim();

    let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let local_version_path = Path::new(&home).join(".memo").join("version.txt");

    let local_version = fs::read_to_string(&local_version_path)
        .unwrap_or_else(|_| "Unknown".to_string());
    let local_version = local_version.trim();

    println!("{}: {}", language::get(lang, "CURRENT_VERSION"), local_version);
    println!("{}: {}", language::get(lang, "SERVER_VERSION"), remote_version);

    if remote_version == local_version {
        println!("{}", language::get(lang, "ALREADY_LATEST_VERSION"));
        return None;
    } else {
        println!(
            "{}: {} -> {}",
            language::get(lang, "NEW_VERSION_AVAILABLE"),
            local_version,
            remote_version
        );

        println!("{}", language::get(lang, "CONFIRM_UPDATE"));

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect(language::get(lang, "FAIL_TO_READ_INPUT"));

        let input = input.trim();

        if input.eq_ignore_ascii_case("y") {
            // 계속
        } else if input.eq_ignore_ascii_case("n") {
            std::process::exit(0);
        } else {
            println!("{}", language::get(lang, "WRONG_ANSWER"));
            return None;
        }
    }

    match get_latest_exe_url() {
        Ok(url) => Some(url),
        Err(e) => {
            eprintln!("{}: {}", language::get(lang, "FAILED_GET_DOWNLOAD_URL"), e);
            None
        }
    }
}

fn get_latest_exe_url() -> Result<String, Box<dyn std::error::Error>> {
    let repo = "githubyoon/Memo";
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::blocking::Client::new();

    let response: Value = client
        .get(&api_url)
        .header(USER_AGENT, "rust-update-checker")
        .send()?
        .json()?;

    let download_url = response["assets"]
        .as_array()
        .and_then(|assets| assets.iter().find(|a| a["name"] == "memo.exe"))
        .and_then(|asset| asset["browser_download_url"].as_str())
        .ok_or("memo.exe not found")?;

    Ok(download_url.to_string())
}

fn download_new(url: &str, lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", language::get(lang, "DOWNLOADING_LATEST_FILE"));

    let mut response = reqwest::blocking::get(url)?;

    let mut file = File::create("memo.exe")?;
    copy(&mut response, &mut file)?;

    println!("{}", language::get(lang, "DOWNLOAD_COMPLETE"));

    update(lang)?;

    if let Ok(content) = fs::read_to_string("version.txt") {
        let remote_version = content.trim();
        let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        let version_path = Path::new(&home).join(".memo").join("version.txt");
        let _ = fs::write(&version_path, remote_version);
    }

    Ok(())
}

fn get_lang() -> String {
    let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let path = Path::new(&home).join(".memo").join("language.txt");

    if let Ok(content) = fs::read_to_string(&path) {
        let lang = content.trim();

        if lang == "kr" || lang == "en" {
            return lang.to_string(); //  정상 값이면 그대로 사용
        }
    }

    "en".to_string() //  실패/이상값만 fallback
}

fn update(lang: &str) -> std::io::Result<()> {
    let home = env::var("USERPROFILE")
        .expect(language::get(lang, "USERPROFILE_NOT_FOUND"));

    let target_path = Path::new(&home).join(".memo").join("src").join("memo.exe");

    let _ = fs::remove_file(&target_path);

    fs::rename("memo.exe", &target_path)?;

    Ok(())
}