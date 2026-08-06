set shell := ["powershell.exe", "-NoProfile", "/c"]
inno :=  "C:\\Users\\henry\\AppData\\Local\\Programs\\Inno Setup 6\\ISCC.exe"

default:
    @just --list

local-deploy:
    cargo install --path crates/cli
    $cfgPath = "$Env:USERPROFILE\.cargo\bin"
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike ("*" + $cfgPath + "*")) {\
    [Environment]::SetEnvironmentVariable("PATH", ("$userPath;$cfgPath"), "User");\
    Write-Host "Added Cargo bin to User PATH." -ForegroundColor Cyan;\
    } else { \
    Write-Host "Cargo bin is already in your PATH." -ForegroundColor Yellow; \
    }

build-inno: 
    cargo install --path crates/cli
    & "{{inno}}" .\installer\setup.iss

local-deploy-inno:
    cargo install --path crates/cli
    & "{{inno}}" .\installer\setup.iss
    .\installer\dist\copper-installer.exe

test-all:
  cargo test -- --nocapture 

test TEST:
  cargo test -p {{TEST}} -- --nocapture 

publish version:
    @$content = Get-Content ".\Cargo.toml" -Raw
    @$new = $content -replace '(?m)^version = ".*"$', 'version = "{{version}}"'
    @Set-Content ".\Cargo.toml" -Value $new
    @cargo check --workspace
    @git add -A
    @git commit -m "release: v{{version}}"
    @git tag v{{version}}
    @git push --follow-tags
