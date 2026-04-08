mod language;

use std::env;
use std::fs::{self, File};
use std::io::{copy, stdin};
use std::path::Path;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use std::io::{self, Write};

fn main() {
    // 1. 원격 버전 정보 다운로드 시도 (실패하면 로컬 버전 사용)
    let _ = download_version_info();

    if let Some(new_url) = check_and_get_url() {
        // 2. 버전이 다르면 실제 memo.exe 다운로드 시작
        if let Err(e) = download_new(&new_url) {
            eprintln!("새 파일 다운로드 실패: {}", e);
        }
    }
}

/// 1. 원격지의 version.txt를 가져오는 함수 (실패해도 무시)
fn download_version_info() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://githubyoon.github.io/Memo/version.txt";
    match reqwest::blocking::get(url) {
        Ok(mut response) => {
            let mut file = File::create("version.txt")?;
            copy(&mut response, &mut file)?;
            println!("원격 버전 정보 업데이트 완료");
            Ok(())
        }
        Err(e) => {
            println!("원격 버전 정보를 가져올 수 없습니다 (로컬 버전 사용): {}", e);
            Ok(())
        }
    }
}

fn check_and_get_url() -> Option<String> {
    let content = fs::read_to_string("version.txt").ok()?;
    let remote_version = content.trim();
    
    let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let local_version_path = Path::new(&home).join(".memo\\version.txt");
    let local_version = fs::read_to_string(&local_version_path).unwrap_or_else(|_| "Unknown".to_string());
    let local_version = local_version.trim();

    println!("현재 버전: {}", local_version);
    println!("원격 버전: {}", remote_version);

    if remote_version == local_version {
        println!("최신 버전입니다!");
        return None;
    } else {
        println!("새로운 버전이 있습니다: {} -> {}", local_version, remote_version);

        println!("업데이트를 할것인가요? (y,n):");

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("입력을 받지 못했습니다");

        let input = input.trim();

        if input.eq_ignore_ascii_case("y") {
            // 계속 진행
        } else if input.eq_ignore_ascii_case("n") {
            std::process::exit(0);
        }
        else {
            println!("잘못된 입력입니다");
            return None;
        }
    }

        // GitHub API를 통해 memo.exe의 실제 주소 찾기
        match get_latest_exe_url() {
            Ok(url) => Some(url),
            Err(e) => {
                eprintln!("GitHub에서 다운로드 URL을 가져올 수 없습니다: {}", e);
                None
            }
        }
    }

/// 3. GitHub API에서 memo.exe의 브라우저 다운로드 URL 추출
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
        .ok_or("API에서 memo.exe 주소를 찾을 수 없습니다.")?;

    Ok(download_url.to_string())
}

/// 4. 실제 전달받은 URL로 memo.exe 파일을 다운로드/덮어쓰기 하는 함수
fn download_new(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("최신 파일을 다운로드 중입니다...");
    let mut response = reqwest::blocking::get(url)?;

    // 파일명을 'memo_new.exe' 등으로 저장한 뒤 나중에 교체하는 게 안전해
    let mut file = File::create("memo.exe")?;
    copy(&mut response, &mut file)?;

    println!("다운로드 완료: memo.exe");
    update()?;
    
    // version.txt 업데이트
    if let Ok(content) = fs::read_to_string("version.txt") {
        let remote_version = content.trim();
        let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        let version_path = Path::new(&home).join(".memo\\version.txt");
        let _ = fs::write(&version_path, remote_version);
    }
    
    Ok(())
}

fn get_lang() -> String {
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    let path = std::path::Path::new(&home).join(".memo/language.txt");

    if let Ok(content) = std::fs::read_to_string(&path) {
        let s = content.trim();
        if !s.is_empty() {
            return s.to_string();
        }
    }

    "kr".to_string()
}

fn update() -> std::io::Result<()> {
    let lang = get_lang();
    let lang = lang.as_str();

    let home = env::var("USERPROFILE")
        .expect(language::get(lang, "USERPROFILE_NOT_FOUND"));

    let target_path = Path::new(&home).join(".memo\\src\\memo.exe");

    // 1. 기존 exe 삭제
    let _ = fs::remove_file(&target_path);

    // 2. updater 위치에 있는 새 파일 → 목표 위치로 이동
    fs::rename("memo.exe", &target_path)?;

    Ok(())
}

fn exit() {
    std::process::exit(0);
}