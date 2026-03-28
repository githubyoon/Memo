pub mod kr;
pub mod en;

pub fn get(lang: &str, key: &str) -> &'static str {
    match lang {
        "kr" => match key {
            "CREATE_FILE_COMPLETE" => kr::CREATE_FILE_COMPLETE,
            "CREATE_FILE_FAIL" => kr::CREATE_FILE_FAIL,
            "USAGE_ADD" => kr::USAGE_ADD,
            "USAGE_DELETE" => kr::USAGE_DELETE,
            "USAGE" => kr::USAGE,
            "MEMO_ADD_FAIL" => kr::MEMO_ADD_FAIL,
            "MEMO_LIST_FAIL" => kr::MEMO_LIST_FAIL,
            "MEMO_DELETE_FAIL" => kr::MEMO_DELETE_FAIL,
            "NEED_NUMBER" => kr::NEED_NUMBER,
            "INVALID_NUMBER" => kr::INVALID_NUMBER,
            "NO_MEMO" => kr::NO_MEMO,
            "USERPROFILE_NOT_FOUND" => kr::USERPROFILE_NOT_FOUND,
            _ => "unknown",
        },
        "en" => match key {
            "CREATE_FILE_COMPLETE" => en::CREATE_FILE_COMPLETE,
            "CREATE_FILE_FAIL" => en::CREATE_FILE_FAIL,
            "USAGE_ADD" => en::USAGE_ADD,
            "USAGE_DELETE" => en::USAGE_DELETE,
            "USAGE" => en::USAGE,
            "MEMO_ADD_FAIL" => en::MEMO_ADD_FAIL,
            "MEMO_LIST_FAIL" => en::MEMO_LIST_FAIL,
            "MEMO_DELETE_FAIL" => en::MEMO_DELETE_FAIL,
            "NEED_NUMBER" => en::NEED_NUMBER,
            "INVALID_NUMBER" => en::INVALID_NUMBER,
            "NO_MEMO" => en::NO_MEMO,
            "USERPROFILE_NOT_FOUND" => en::USERPROFILE_NOT_FOUND,
            _ => "unknown",
        },
        _ => "unknown",
    }
}

pub fn memo_added(lang: &str, id: usize, content: &str) -> String {
    match lang {
        "kr" => format!("{}번 메모 추가됨: {}", id, content),
        "en" => format!("Memo {} added: {}", id, content),
        _ => "unknown".to_string(),
    }
}

pub fn memo_deleted(lang: &str, id: usize) -> String {
    match lang {
        "kr" => format!("{}번 메모 삭제됨", id),
        "en" => format!("Memo {} deleted", id),
        _ => "unknown".to_string(),
    }
}