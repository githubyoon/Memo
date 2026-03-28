use std::path::Path;
use std::fs;
use std::env;
use std::io;

fn main() {
    let home = env::var("USERPROFILE").expect("USERPROFILE 환경변수 없음");
    let path = Path::new(&home).join("Memo.json");
    let version = "0.2.0";

    if !path.exists() {
        fs::write(&path, "[]").expect("파일 생성 실패");
        println!("파일 생성 완료");
    }

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let content = get_arg(&args, 2, "사용법: memo add <내용>");
            memo_add(&path, &content).expect("메모 추가 실패");
        }
        Some("list") => {
            memo_list(&path).expect("메모 목록 실패");
        }
        Some("delete") => {
            let index = get_arg(&args, 2, "사용법: memo delete <번호>")
                .parse::<usize>()
                .unwrap_or_else(|_| {
                    println!("숫자를 입력해줘");
                    std::process::exit(1);
                });
            memo_delete(&path, index).expect("메모 삭제 실패");
        }
        Some("version") => {
            println!("Memo {}" , version);
        }
        _ => {
            println!("사용법:");
            println!("  memo add <내용>");
            println!("  memo list");
            println!("  memo delete <번호>");
            println!("  memo version")
        }
    }
}

fn get_arg(args: &[String], index: usize, msg: &str) -> String {
    args.get(index).cloned().unwrap_or_else(|| {
        println!("{}", msg);
        std::process::exit(1);
    })
}

fn memo_add(path: &Path, content: &str) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let mut memos: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();

    let id = memos.len() + 1;
    memos.push(serde_json::json!({
        "id": id,
        "content": content
    }));

    fs::write(path, serde_json::to_string_pretty(&memos).unwrap())?;
    println!("{}번 메모 추가됨: {}", id, content);
    Ok(())
}

fn memo_list(path: &Path) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let memos: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();

    if memos.is_empty() {
        println!("메모가 없어요");
        return Ok(());
    }

    for memo in &memos {
        println!("{}번: {}", memo["id"], memo["content"]);
    }
    Ok(())
}

fn memo_delete(path: &Path, index: usize) -> io::Result<()> {
    let data = fs::read_to_string(path)?;
    let mut memos: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();

    if index == 0 || index > memos.len() {
        println!("없는 번호예요");
        return Ok(());
    }

    memos.remove(index - 1);

    // 삭제 후 id 재정렬
    for (i, memo) in memos.iter_mut().enumerate() {
        memo["id"] = serde_json::json!(i + 1);
    }

    fs::write(path, serde_json::to_string_pretty(&memos).unwrap())?;
    println!("{}번 메모 삭제됨", index);
    Ok(())
}