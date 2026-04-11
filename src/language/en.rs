pub const CREATE_FILE_FAIL: &str = "File creation failed";
pub const CREATE_FILE_COMPLETE: &str = "File creation complete";
pub const CREATE_DIR_FAIL: &str = "Failed to create directory.";
pub const USAGE_ADD: &str = "Usage: memo add <text>";
pub const USAGE_DELETE: &str = "Usage: memo delete <index>";

// tab 한번 backspace 한번
pub const USAGE: &str = "Usage:\n  memo add <text>\n  memo list\n  memo delete <index>\n  memo version\n  memo settings <setting>\n  memo reset\n  memo repo | github\n";

pub const MEMO_ADD_FAIL: &str = "Failed to add memo";
pub const MEMO_LIST_FAIL: &str = "Failed to list memos";
pub const MEMO_DELETE_FAIL: &str = "Failed to delete memo";

pub const NEED_NUMBER: &str = "Please enter a number";
pub const INVALID_NUMBER: &str = "Invalid number";
pub const NO_MEMO: &str = "No memos";

pub const USERPROFILE_NOT_FOUND: &str = "USERPROFILE environment variable not found";

pub const USAGE_SETTINGS: &str = "Usage \n  memo settings language <language>\n  memo update";

pub const LANGUAGE_LIST: &str = "Available languages:\n    English <en>\n   Korean<kr>";

pub const SETTINGS_SAVED: &str = "Settings saved";
pub const SETTINGS_SAVE_FAIL: &str = "Failed to save settings";
pub const INVALID_LANG: &str = "Invalid language";
pub const USAGE_SETTINGS_LANG: &str = "Usage: memo settings [language | lang] [kr|en]";
pub const CHECK_MEMO_RESET: &str = "Are you sure you want to delete all memos? (y/n)";
pub const RESET_FAIL: &str = "Failed to reset memos.";
pub const RESET_SUCCESS: &str = "All memos have been deleted.";
pub const FAIL_TO_READ_INPUT: &str = "Failed to read input.";
pub const WRONG_ANSWER: &str = "Invalid input.";

pub const FAIL_TO_START: &str = "Failed to start updater";

// updater (en)

pub const FAIL_TO_DOWNLOAD: &str = "Failed to download the new file";
pub const FETCHED_LATEST_VERSION_MSG: &str = "Fetched the latest version information";

pub const FETCH_VERSION_FAILED_USE_LOCAL_MSG: &str =
    "Failed to fetch version info from server, using local version";

pub const CURRENT_VERSION: &str = "Current version";
pub const SERVER_VERSION: &str = "Server version";

pub const ALREADY_LATEST_VERSION_MSG: &str = "Already up to date";

pub const NEW_VERSION_AVAILABLE_MSG: &str = "A new version is available";

pub const CONFIRM_UPDATE_MSG: &str = "Do you want to update? (y/n)";

pub const FAILED_GET_DOWNLOAD_URL: &str =
    "Failed to get download URL from GitHub";

pub const DOWNLOADING_LATEST_FILE: &str = "Downloading the latest file...";
pub const DOWNLOAD_COMPLETE: &str = "Download complete";