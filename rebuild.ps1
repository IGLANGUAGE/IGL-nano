# Очистка проекта
Remove-Item -Recurse -Force public -ErrorAction SilentlyContinue
cargo clean

# Создаем структуру папок
New-Item -ItemType Directory -Path public

# Сборка проекта
cargo build --release --target wasm32-unknown-unknown

# Проверяем наличие wasm-opt
$WasmOpt = Get-Command wasm-opt -ErrorAction SilentlyContinue

# Оптимизируем WASM
if ($WasmOpt) {
    wasm-opt -Oz target/wasm32-unknown-unknown/release/igl_nano.wasm -o public/out.wasm
} else {
    Copy-Item target/wasm32-unknown-unknown/release/igl_nano.wasm public/out.wasm
}

# Создаем HTML-тестер
@'
<!DOCTYPE html>
<html>
<head>
    <title>IGL-Nano Test</title>
    <meta charset="UTF-8">
    <script>
    // Минимальные необходимые импорты
    const imports = {
        env: {
            memory: new WebAssembly.Memory({ initial: 1, maximum: 1 }),
            __memory_base: 0,
            __table_base: 0,
            table: new WebAssembly.Table({ initial: 0, element: 'anyfunc' })
        }
    };

    // Загрузка WASM
    async function loadWasm() {
        try {
            const response = await fetch('out.wasm');
            const bytes = await response.arrayBuffer();
            const { instance } = await WebAssembly.instantiate(bytes, imports);
            
            // Тестируем функцию
            console.log("2 + 3 =", instance.exports.add(2, 3));
        } catch (e) {
            console.error("WASM Error:", e);
        }
    }
    
    // Запускаем при загрузке страницы
    window.onload = loadWasm;
    </script>
</head>
<body>
    <h1>IGL-Nano Test Page</h1>
    <p>Check browser console (F12)</p>
</body>
</html>
'@ | Out-File -Encoding utf8 public/index.html

Write-Host "Rebuild complete! Run: cd public; python -m http.server 8000"