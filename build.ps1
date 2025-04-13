# Очистка
Remove-Item -Recurse -Force public -ErrorAction SilentlyContinue
cargo clean

# Создаем папки
New-Item -ItemType Directory -Path public

# Сборка
cargo build --release --target wasm32-unknown-unknown

# Копируем WASM
Copy-Item target/wasm32-unknown-unknown/release/igl_nano.wasm public/out.wasm

# Создаем HTML с правильной настройкой памяти
@'
<!DOCTYPE html>
<html>
<head>
    <title>IGL-Nano Test</title>
    <script>
    const imports = {
        env: {
            memory: new WebAssembly.Memory({
                initial: 1,
                maximum: 1  // Явно указываем максимальный размер
            })
        }
    };
    
    WebAssembly.instantiateStreaming(fetch("out.wasm"), imports)
      .then(obj => {
          console.log("2 + 3 =", obj.instance.exports.add(2, 3));
      })
      .catch(e => console.error("Error:", e));
    </script>
</head>
<body>
    <h1>IGL-Nano Test Page</h1>
    <p>Open browser console (F12) to see results</p>
</body>
</html>
'@ | Out-File -Encoding utf8 public/index.html

Write-Host "Build complete! Run: cd public; python -m http.server 8000"