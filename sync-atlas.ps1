# Sync Dubai Data to MongoDB Atlas
# Executa o binário que envia os dados locais para o banco

param(
    [switch]$Build,
    [switch]$Release
)

Write-Host "🌐 Dubai Data Sync - MongoDB Atlas" -ForegroundColor Cyan
Write-Host ""

# Check if .env exists
if (-not (Test-Path ".env")) {
    Write-Host "❌ Arquivo .env não encontrado!" -ForegroundColor Red
    Write-Host "💡 Copie .env.example para .env e configure as variáveis" -ForegroundColor Yellow
    exit 1
}

# Load environment variables from .env
Write-Host "📋 Carregando variáveis de ambiente..." -ForegroundColor Yellow
Get-Content .env | ForEach-Object {
    if ($_ -match '^([^#][^=]+)=(.*)$') {
        $key = $matches[1].Trim()
        $value = $matches[2].Trim()
        [Environment]::SetEnvironmentVariable($key, $value, "Process")
    }
}

# Check MongoDB Atlas configuration
$required_vars = @(
    "MONGO_ATLAS_URI",
    "MONGODB_ATLAS_APP_ID",
    "MONGODB_ATLAS_API_KEY",
    "MONGODB_ATLAS_CLUSTER",
    "MONGODB_ATLAS_DATABASE",
    "MONGODB_ATLAS_COLLECTION"
)

$missing_vars = @()
foreach ($var in $required_vars) {
    if (-not [Environment]::GetEnvironmentVariable($var, "Process")) {
        $missing_vars += $var
    }
}

if ($missing_vars.Count -gt 0) {
    Write-Host "❌ Variáveis de ambiente faltando:" -ForegroundColor Red
    foreach ($var in $missing_vars) {
        Write-Host "   - $var" -ForegroundColor Red
    }
    Write-Host ""
    Write-Host "💡 Configure estas variáveis no arquivo .env" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Configuração validada" -ForegroundColor Green
Write-Host ""

# Build if requested
if ($Build -or $Release) {
    Write-Host "🔨 Compilando..." -ForegroundColor Yellow
    
    if ($Release) {
        cargo build --release --bin sync-to-atlas
        $binary_path = "target/release/sync-to-atlas.exe"
    } else {
        cargo build --bin sync-to-atlas
        $binary_path = "target/debug/sync-to-atlas.exe"
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Erro ao compilar" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ Compilação concluída" -ForegroundColor Green
    Write-Host ""
} else {
    # Use existing binary
    $binary_path = "target/debug/sync-to-atlas.exe"
    if (-not (Test-Path $binary_path)) {
        $binary_path = "target/release/sync-to-atlas.exe"
        if (-not (Test-Path $binary_path)) {
            Write-Host "❌ Binário não encontrado!" -ForegroundColor Red
            Write-Host "💡 Execute com -Build para compilar primeiro" -ForegroundColor Yellow
            exit 1
        }
    }
}

# Run the sync
Write-Host "🚀 Executando sincronização..." -ForegroundColor Cyan
Write-Host ""

& $binary_path

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Sincronização concluída com sucesso!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📊 Acesse MongoDB Atlas:" -ForegroundColor Cyan
    Write-Host "   https://cloud.mongodb.com" -ForegroundColor White
    Write-Host ""
    Write-Host "🔍 Database: $env:MONGODB_ATLAS_DATABASE" -ForegroundColor Yellow
    Write-Host "📦 Collection: $env:MONGODB_ATLAS_COLLECTION" -ForegroundColor Yellow
} else {
    Write-Host ""
    Write-Host "❌ Erro na sincronização" -ForegroundColor Red
    exit 1
}
