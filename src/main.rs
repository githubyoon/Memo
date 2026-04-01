mod language;

use std::path::Path;
use std::fs;
use std::env;
use std::io;
use std::io::stdin;

fn main() {
    let lang = get_lang();
    let lang = lang.as_str();

    let home = env::var("USERPROFILE")
        .expect(language::get(lang, "USERPROFILE_NOT_FOUND"));
    let memo_dir = Path::new(&home).join(".memo");

    fs::create_dir_all(&memo_dir)
        .expect(language::get(lang, "CREATE_DIR_FAIL"));

    let path = memo_dir.join("Memo.json");


    if !path.exists() {
        fs::write(&path, "[]")
            .expect(language::get(lang, "CREATE_FILE_FAIL"));
        println!("{}", language::get(lang, "CREATE_FILE_COMPLETE"));
    }

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let content = get_arg(&args, 2, language::get(lang, "USAGE_ADD"));
            memo_add(&path, &content, lang)
                .expect(language::get(lang, "MEMO_ADD_FAIL"));
        }
        Some("list") => {
            memo_list(&path, lang)
                .expect(language::get(lang, "MEMO_LIST_FAIL"));
        }
        Some("delete") | Some("del") => {
            let index = get_arg(&args, 2, language::get(lang, "USAGE_DELETE"))
                .parse::<usize>()
                .unwrap_or_else(|_| {
                    println!("{}", language::get(lang, "NEED_NUMBER"));
                    std::process::exit(1);
                });

            memo_delete(&path, index, lang)
                .expect(language::get(lang, "MEMO_DELETE_FAIL"));
        }
        Some("version") => {
            version();
        }
        Some("-v") => {
            version();
        }
        Some("ver") => {
            version();
        }
        Some("settings") => {
            match args.get(2).map(|s| s.as_str()) {
                Some("language") => {
                    let value = get_arg(&args, 3, language::get(lang, "USAGE_SETTINGS_LANG"));

                    match value.as_str() {
                        "kr" | "en" => {
                            let home = env::var("USERPROFILE")
                                .expect(language::get(lang, "USERPROFILE_NOT_FOUND"));

                            let memo_dir = Path::new(&home).join(".memo");

                            fs::create_dir_all(&memo_dir)
                                .expect(language::get(lang, "CREATE_DIR_FAIL"));

                            let lang_path = memo_dir.join("language.txt");

                            fs::write(&lang_path, &value)
                                .expect(language::get(lang, "SETTINGS_SAVE_FAIL"));

                            println!("{}", language::get(&value, "SETTINGS_SAVED"));
                        }
                        _ => {
                            println!("{}", language::get(lang, "INVALID_LANG"));
                        }
                    }
                }
                _ => {  // "lang"이 아니거나, 아예 인자가 없을 때
                    println!("{}", language::get(lang, "USAGE_SETTINGS"));
                }
            }
        }
        Some("reset") => {
            println!("{}", language::get(lang, "CHECK_MEMO_RESET"));

            let mut input = String::new();

            stdin()
                .read_line(&mut input)
                .expect(language::get(lang, "FAIL_TO_READ_INPUT"));

            let input = input.trim(); // 🔥 그 다음 trim

            if input.eq_ignore_ascii_case("y") {
                memo_reset(&path, lang);
            } else if input.eq_ignore_ascii_case("n") {
                return;
            }
            else {
                println!("{}", language::get(lang, "WRONG_ANSWER"));
            }
        }

        _ => {
            println!("{}", language::get(lang, "USAGE"));
        }
    }
}


fn get_lang() -> String {
    let home = env::var("USERPROFILE")
        .unwrap_or_else(|_| ".".to_string());

    let memo_dir = Path::new(&home).join(".memo");

    // 폴더 없으면 생성
    fs::create_dir_all(&memo_dir).ok();

    let lang_path = memo_dir.join("language.txt");

    // 파일 없으면 기본값 생성
    if !lang_path.exists() {
        fs::write(&lang_path, "kr").ok();
    }

    fs::read_to_string(&lang_path)
        .unwrap_or("kr".to_string())
        .trim()
        .to_string()
}

fn get_arg(args: &[String], index: usize, msg: &str) -> String {
    args.get(index).cloned().unwrap_or_else(|| {
        println!("{}", msg);
        std::process::exit(1);
    })
}

fn memo_add(path: &Path, content: &str, lang: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let mut memos: Vec<serde_json::Value> = serde_json::from_str(&data)?;

    let id = memos.len() + 1;
    memos.push(serde_json::json!({
        "id": id,
        "content": content
    }));

    fs::write(path, serde_json::to_string_pretty(&memos)?)?;
    println!("{}", language::memo_added(lang, id, content));
    Ok(())
}

fn memo_list(path: &Path, lang: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let memos: Vec<serde_json::Value> = serde_json::from_str(&data)?;

    if memos.is_empty() {
        println!("{}", language::get(lang, "NO_MEMO"));
        return Ok(());
    }

    for memo in &memos {
        println!("{}: {}", memo["id"], memo["content"]);
    }
    Ok(())
}

fn memo_delete(path: &Path, index: usize, lang: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let mut memos: Vec<serde_json::Value> = serde_json::from_str(&data)?;

    if index == 0 || index > memos.len() {
        println!("{}", language::get(lang, "INVALID_NUMBER"));
        return Ok(());
    }

    memos.remove(index - 1);

    for (i, memo) in memos.iter_mut().enumerate() {
        memo["id"] = serde_json::json!(i + 1);
    }

    fs::write(path, serde_json::to_string_pretty(&memos)?)?;
    println!("{}", language::memo_deleted(lang, index));
    Ok(())
}
fn version() {
    let version = "Beta 0.5.3";
    println!("Memo ({})", version);
}
fn memo_reset(path: &std::path::Path, lang: &str) {
    if let Err(_) = std::fs::write(path, "[]") {
        println!("{}", language::get(lang, "RESET_FAIL"));
        return;
    }

    println!("{}", language::get(lang, "RESET_SUCCESS"));
}