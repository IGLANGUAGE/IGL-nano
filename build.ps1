param (
    [ValidateSet("release", "docs", "dev")]
    [string]$Target = "release"
)

function Build-Wasm {
    if ($Target -eq "release") {
        # Отключаем LTO для WASM
        $env:RUSTFLAGS = "-C embed-bitcode=no"
        cargo build --release --target wasm32-unknown-unknown
    } else {
        cargo build --target wasm32-unknown-unknown
    }
    if (-not $?) { exit 1 }
    Write-Host "WASM build completed" -ForegroundColor Green
}

function Generate-Docs {
    cargo doc --no-deps --open
    Write-Host "Documentation generated" -ForegroundColor Green
}

switch ($Target) {
    { $_ -in "release", "dev" } { Build-Wasm }
    "docs" { Generate-Docs }
}