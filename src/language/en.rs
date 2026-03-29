pub const CREATE_FILE_FAIL: &str = "File creation failed";
pub const CREATE_FILE_COMPLETE: &str = "File creation complete";

pub const USAGE_ADD: &str = "Usage: memo add <text>";
pub const USAGE_DELETE: &str = "Usage: memo delete <index>";
pub const USAGE: &str = "Usage:\n  memo add <text>\n  memo list\n  memo delete <index>\n  memo version\n  memo settings <setting>";

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