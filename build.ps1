param (
    [ValidateSet("release", "docs", "test")]
    [string]$Target = "release"
)

function Build-Wasm {
    cargo build --release --target wasm32-unknown-unknown
    if (-not $?) { exit 1 }
    $size = (Get-Item "target/wasm32-unknown-unknown/release/igl_nano.wasm").Length / 1KB
    Write-Host "WASM size: $($size.ToString('0.00')) KB" -ForegroundColor Green
}

function Generate-Docs {
    cargo doc --no-deps --open
}

function Run-Tests {
    cargo test --target wasm32-wasi -- --nocapture
}

switch ($Target) {
    "release" { Build-Wasm }
    "docs" { Generate-Docs }
    "test" { Run-Tests }
}