@echo off

:: ===============================
:: 관리자 권한 요청
:: ===============================
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"

if '%errorlevel%' NEQ '0' (
    echo Requesting administrative privileges...
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "%~s0", "", "", "runas", 1 >> "%temp%\getadmin.vbs"
    "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    if exist "%temp%\getadmin.vbs" del "%temp%\getadmin.vbs"
    pushd "%CD%"
    cd /d "%~dp0"

:: ===============================
:: 설정
:: ===============================
set REPO=githubyoon/Memo
set INSTALL_DIR=%USERPROFILE%\.memo
set SRC_DIR=%INSTALL_DIR%\src
set BIN_DIR=%INSTALL_DIR%\bin

set UPDATER_URL=https://github.com/githubyoon/Memo/releases/download/updater/updater.exe

:: ===============================
:: memo URL 가져오기 (API)
:: ===============================
echo 최신 memo.exe URL 가져오는 중...

for /f "delims=" %%i in ('powershell -NoProfile -Command "(Invoke-RestMethod https://api.github.com/repos/%REPO%/releases/latest).assets | Where-Object {$_.name -like '*memo.exe*'} | Select-Object -ExpandProperty browser_download_url"') do set "URL=%%i"

echo URL: %URL%

if "%URL%"=="" (
    echo [ERROR] memo.exe URL 못 찾음
    pause
    exit /b
)

:: ===============================
:: memo 다운로드
:: ===============================
echo memo 다운로드 중...
curl -L -o memo.exe "%URL%"

if not exist memo.exe (
    echo [ERROR] 다운로드 실패
    pause
    exit /b
)

:: ===============================
:: 초기 실행 (폴더 생성)
:: ===============================
echo 초기 실행...
start "" /wait memo.exe

:: ===============================
:: 폴더 생성
:: ===============================
if not exist "%SRC_DIR%" mkdir "%SRC_DIR%"
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

:: ===============================
:: 이동
:: ===============================
move /Y memo.exe "%SRC_DIR%\memo.exe"

:: ===============================
:: updater 다운로드
:: ===============================
echo updater 다운로드 중...
curl -L -o "%BIN_DIR%\updater.exe" "%UPDATER_URL%"

:: ===============================
:: PATH 등록
:: ===============================
echo %PATH% | find /I "%BIN_DIR%" >nul
if %errorlevel% NEQ 0 (
    setx PATH "%BIN_DIR%;%PATH%" >nul
    echo PATH 추가 완료 (새 콘솔에서 적용됨)
) else (
    echo PATH 이미 존재
)

:: ===============================
:: 실행
:: ===============================
echo 실행!
start "" "%SRC_DIR%\memo.exe"

echo 완료!
pause