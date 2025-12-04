# Deploy Dubai Project na Avila Cloud
# Script automatizado usando infraestrutura propria

$ErrorActionPreference = "Stop"

Write-Host "Deploy Dubai Project para Avila Cloud" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan

# Configuracoes
$AVILA_CLI = "d:\arxis\target\release\avila-cloud-cli.exe"
$INSTANCE_NAME = "dubai-production"
$BUCKET_NAME = "dubai-assets"
$VPC_NAME = "dubai-network"

# 1. Build do projeto
Write-Host "`nBuilding Dubai Project..." -ForegroundColor Yellow
cd "d:\01 - Dubai"
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build falhou!" -ForegroundColor Red
    exit 1
}
Write-Host "Build concluido!" -ForegroundColor Green

# 2. Verificar binario
Write-Host "`nVerificando binario..." -ForegroundColor Yellow
$binaryPath = "target\release\dubai-project.exe"
if (Test-Path $binaryPath) {
    $sizeMB = [math]::Round((Get-Item $binaryPath).Length/1MB, 2)
    Write-Host "Binario encontrado: $binaryPath ($sizeMB MB)" -ForegroundColor Green
}

# 3. Verificar instancia
Write-Host "`nVerificando instancia na Avila Cloud..." -ForegroundColor Yellow
& $AVILA_CLI compute list

# 4. Status da infraestrutura
Write-Host "`nStatus da Infraestrutura:" -ForegroundColor Cyan
Write-Host "Instancia: $INSTANCE_NAME (t3.xlarge)"
Write-Host "Bucket:    $BUCKET_NAME"
Write-Host "VPC:       $VPC_NAME (10.100.0.0/16)"

# 5. Proximos passos
Write-Host "`nProximos Passos:" -ForegroundColor Yellow
Write-Host "1. Configurar DNS: dubai.avilaops.com"
Write-Host "2. Fazer upload do binario"
Write-Host "3. Iniciar container"

Write-Host "`nPreparacao concluida!" -ForegroundColor Green
Write-Host "URLs: https://dubai.avilaops.com:8080" -ForegroundColor Cyan
