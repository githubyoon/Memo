mod language;

use std::path::Path;
use std::fs;
use std::env;
use std::io;

fn main() {
    let lang = get_lang();
    let lang = lang.as_str();

    let home = env::var("USERPROFILE")
        .expect(language::get(lang, "USERPROFILE_NOT_FOUND"));

    let path = Path::new(&home).join("Memo.json");
    let version = "0.3.0";

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
        Some("delete") => {
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
            println!("Memo {}", version);
        }
        _ => {
            println!("{}", language::get(lang, "USAGE"));
        }
    }
}

fn get_lang() -> String {
    let path = "language.txt";

    if !std::path::Path::new(path).exists() {
        fs::write(path, "kr").ok();
    }

    fs::read_to_string(path)
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