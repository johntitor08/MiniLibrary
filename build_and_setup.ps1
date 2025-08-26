Write-Host "🚀 Rust Release Build başlatılıyor..." -ForegroundColor Green
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Derleme başarısız!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Derleme tamamlandı. EXE: target\release\mini-library.exe" -ForegroundColor Green

$inno = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe"
if (-Not (Test-Path $inno)) {
    Write-Host "❌ Inno Setup bulunamadı. Lütfen Inno Setup 6 kur." -ForegroundColor Red
    exit 1
}

Write-Host "📦 Setup oluşturuluyor..." -ForegroundColor Yellow
& "$inno" "installer.iss"

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Setup oluşturma başarısız!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Setup başarıyla oluşturuldu: Output\MiniLibrarySetup.exe" -ForegroundColor Cyan

# Sessiz kurulum ve tek çalıştırma
$installer = "Output\MiniLibrarySetup.exe"
$appPath = "${env:ProgramFiles(x86)}\MiniLibrary\mini-library.exe"

Write-Host "⚡ Sessiz kurulum başlatılıyor..." -ForegroundColor Yellow
Start-Process -FilePath $installer -ArgumentList "/VERYSILENT /NORESTART" -Wait

if (Test-Path $appPath) {
    Write-Host "🎉 Kurulum tamamlandı! Uygulama başlatılıyor..." -ForegroundColor Green
    Start-Process -FilePath $appPath
}
else {
    Write-Host "❌ Kurulum sonrası uygulama bulunamadı!" -ForegroundColor Red
}

exit 0
