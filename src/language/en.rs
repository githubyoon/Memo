pub const CREATE_FILE_FAIL: &str = "File creation failed";
pub const CREATE_FILE_COMPLETE: &str = "File creation complete";
pub const CREATE_DIR_FAIL: &str = "Failed to create directory.";
pub const USAGE_ADD: &str = "Usage: memo add <text>";
pub const USAGE_DELETE: &str = "Usage: memo delete <index>";

// tab 한번 backspace 한번
pub const USAGE: &str = "Usage:\n  memo add <text>\n  memo list\n  memo delete <index>\n  memo version\n  memo settings <setting>\n  memo reset\n  memo repo | github";

pub const MEMO_ADD_FAIL: &str = "Failed to add memo";
pub const MEMO_LIST_FAIL: &str = "Failed to list memos";
pub const MEMO_DELETE_FAIL: &str = "Failed to delete memo";

pub const NEED_NUMBER: &str = "Please enter a number";
pub const INVALID_NUMBER: &str = "Invalid number";
pub const NO_MEMO: &str = "No memos";

pub const USERPROFILE_NOT_FOUND: &str = "USERPROFILE environment variable not found";

pub const USAGE_SETTINGS: &str = "Usage \n  memo settings language <language>";

pub const LANGUAGE_LIST: &str = "Available languages:\n    English <en>\n   Korean<kr>";

pub const SETTINGS_SAVED: &str = "Settings saved";
pub const SETTINGS_SAVE_FAIL: &str = "Failed to save settings";
pub const INVALID_LANG: &str = "Invalid language";
pub const USAGE_SETTINGS_LANG: &str = "Usage: memo settings language [kr|en]";
pub const CHECK_MEMO_RESET: &str = "Are you sure you want to delete all memos? (y/n)";
pub const RESET_FAIL: &str = "Failed to reset memos.";
pub const RESET_SUCCESS: &str = "All memos have been deleted.";
pub const FAIL_TO_READ_INPUT: &str = "Failed to read input.";
pub const WRONG_ANSWER: &str = "Invalid input.";