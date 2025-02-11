@echo off
set "TARGET_FOLDER=pkg"

REM Check if the folder exists & delete it
if exist "%TARGET_FOLDER%" (
    rmdir /s /q "%TARGET_FOLDER%"
    echo Folder deleted: %TARGET_FOLDER%.
) else (
    echo Folder does not exist: %TARGET_FOLDER%.
)

REM Clean previous build
cargo clean

REM Build Rust to WASM
cargo build --target wasm32-unknown-unknown --release
if errorlevel 1 (
    echo Failed to build Rust to WASM.
    exit /b 1
)

REM Generate WASM bindings
wasm-bindgen --target web --out-dir "%TARGET_FOLDER%" --no-typescript target\wasm32-unknown-unknown\release\snek_hq_extension.wasm
if errorlevel 1 (
    echo Failed to generate WASM bindings.
    exit /b 1
)

REM Copy necessary files
for %%f in (index.html popup.js background.js manifest.json styles.css) do (
    copy "%%f" "%TARGET_FOLDER%"
    if errorlevel 1 (
        echo Failed to copy %%f.
        exit /b 1
    )
)

REM Create icons folder if it doesn't exist
set "ICONS_FOLDER=%TARGET_FOLDER%\icons"
if not exist "%ICONS_FOLDER%" (
    mkdir "%ICONS_FOLDER%"
    echo Folder created at %ICONS_FOLDER%.
) else (
    echo Folder already exists at %ICONS_FOLDER%.
)

REM Copy icon files
for %%f in (snek_icon_16.png snek_icon_48.png snek_icon_128.png) do (
    copy "%%f" "%ICONS_FOLDER%"
    if errorlevel 1 (
        echo Failed to copy %%f.
        exit /b 1
    )
)

echo Build complete! Files are in %TARGET_FOLDER%\ directory.