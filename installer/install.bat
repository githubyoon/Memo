@echo off

:: ===============================
:: Request Administrator Privileges
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
:: Configuration
:: ===============================
set REPO=githubyoon/Memo
set INSTALL_DIR=%USERPROFILE%\.memo
set SRC_DIR=%INSTALL_DIR%\src
set BIN_DIR=%INSTALL_DIR%\bin

set UPDATER_URL=https://github.com/githubyoon/Memo/releases/download/updater/updater.exe

:: ===============================
:: Fetch latest memo.exe URL
:: ===============================
echo Fetching latest memo.exe URL...

for /f "delims=" %%i in ('powershell -NoProfile -Command "(Invoke-RestMethod https://api.github.com/repos/%REPO%/releases/latest).assets | Where-Object {$_.name -like '*memo.exe*'} | Select-Object -ExpandProperty browser_download_url"') do set "URL=%%i"

echo URL: %URL%

if "%URL%"=="" (
    echo [ERROR] Failed to find memo.exe URL
    pause
    exit /b
)

:: ===============================
:: Download memo.exe
:: ===============================
echo Downloading memo...
curl -L -o memo.exe "%URL%"

if not exist memo.exe (
    echo [ERROR] Download failed
    pause
    exit /b
)

:: ===============================
:: First run (initial setup)
:: ===============================
echo Running initial setup...
start "" /wait memo.exe

:: ===============================
:: Create directories
:: ===============================
if not exist "%SRC_DIR%" mkdir "%SRC_DIR%"
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

:: ===============================
:: Move file
:: ===============================
move /Y memo.exe "%SRC_DIR%\memo.exe"

:: ===============================
:: Download updater
:: ===============================
echo Downloading updater...
curl -L -o "%BIN_DIR%\updater.exe" "%UPDATER_URL%"

:: ===============================
:: Add to PATH
:: ===============================
echo %PATH% | find /I "%BIN_DIR%" >nul
if %errorlevel% NEQ 0 (
    setx PATH "%BIN_DIR%;%PATH%" >nul
    echo PATH updated (available in new terminal)
) else (
    echo PATH already exists
)

:: ===============================
:: Run application
:: ===============================
echo Launching memo...
start "" "%SRC_DIR%\memo.exe"

echo Done!
pause