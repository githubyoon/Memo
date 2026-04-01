pub const CREATE_FILE_FAIL: &str = "파일 생성 실패";
pub const CREATE_FILE_COMPLETE: &str = "파일 생성 완료";
pub const CREATE_DIR_FAIL: &str = "폴더 생성에 실패했습니다.";

pub const USAGE_ADD: &str = "사용법: memo add <내용>";
pub const USAGE_DELETE: &str = "사용법: memo delete <번호>";

// tab 한번 space 한번
pub const USAGE: &str = "사용법:\n  memo add <내용>\n  memo list\n  memo delete <번호>\n  memo version\n  memo settings <설정>\n  memo reset";

pub const MEMO_ADD_FAIL: &str = "메모 추가 실패";
pub const MEMO_LIST_FAIL: &str = "메모 목록 실패";
pub const MEMO_DELETE_FAIL: &str = "메모 삭제 실패";

pub const NEED_NUMBER: &str = "숫자를 입력해줘";
pub const INVALID_NUMBER: &str = "없는 번호예요";
pub const NO_MEMO: &str = "메모가 없어요";

pub const USERPROFILE_NOT_FOUND: &str = "USERPROFILE 환경변수 없음";

pub const USAGE_SETTINGS: &str = "사용법 \n  memo settings language <언어>";

pub const LANGUAGE_LIST: &str = "사용 가능 언어:\n    영어 <en>\n   한국어<kr>";

pub const SETTINGS_SAVED: &str = "설정이 저장되었습니다";
pub const SETTINGS_SAVE_FAIL: &str = "설정 저장 실패";
pub const INVALID_LANG: &str = "지원하지 않는 언어입니다";
pub const USAGE_SETTINGS_LANG: &str = "사용법: memo settings language [kr|en]";

pub const CHECK_MEMO_RESET: &str = "모든 메모를 삭제하시겠습니까? (y/n)";
pub const RESET_FAIL: &str = "메모 초기화에 실패했습니다.";
pub const RESET_SUCCESS: &str = "모든 메모가 삭제되었습니다.";
pub const FAIL_TO_READ_INPUT: &str = "입력을 읽는 데 실패했습니다.";
pub const WRONG_ANSWER: &str = "잘못된 입력입니다.";