# Fetch Dubai Real Estate Data from APIs
# Pulls data from government and property portal APIs

param(
    [switch]$Build,
    [switch]$Release,
    [switch]$DryRun
)

Write-Host "🏙️ Dubai Data Fetcher - API Integration" -ForegroundColor Cyan
Write-Host ""

# Check if .env exists
if (-not (Test-Path ".env")) {
    Write-Host "❌ Arquivo .env não encontrado!" -ForegroundColor Red
    exit 1
}

# Load environment variables
Write-Host "📋 Carregando configuração..." -ForegroundColor Yellow
Get-Content .env | ForEach-Object {
    if ($_ -match '^([^#][^=]+)=(.*)$') {
        $key = $matches[1].Trim()
        $value = $matches[2].Trim()
        [Environment]::SetEnvironmentVariable($key, $value, "Process")
    }
}

# Check API credentials
Write-Host ""
Write-Host "🔑 Verificando credenciais de API..." -ForegroundColor Yellow
Write-Host ""

$apis_available = @()
$apis_missing = @()

# Bayut
if ([Environment]::GetEnvironmentVariable("BAYUT_API_KEY", "Process")) {
    $apis_available += "Bayut"
    Write-Host "   ✅ Bayut API configurada" -ForegroundColor Green
} else {
    $apis_missing += "Bayut"
    Write-Host "   ⚠️ Bayut API não configurada" -ForegroundColor Yellow
}

# Property Finder
if ([Environment]::GetEnvironmentVariable("PROPERTYFINDER_CLIENT_ID", "Process") -and 
    [Environment]::GetEnvironmentVariable("PROPERTYFINDER_CLIENT_SECRET", "Process")) {
    $apis_available += "PropertyFinder"
    Write-Host "   ✅ Property Finder API configurada" -ForegroundColor Green
} else {
    $apis_missing += "PropertyFinder"
    Write-Host "   ⚠️ Property Finder API não configurada" -ForegroundColor Yellow
}

# DLD
if ([Environment]::GetEnvironmentVariable("DLD_API_KEY", "Process")) {
    $apis_available += "DLD"
    Write-Host "   ✅ Dubai Land Department API configurada" -ForegroundColor Green
} else {
    $apis_missing += "DLD"
    Write-Host "   ⚠️ Dubai Land Department API não configurada" -ForegroundColor Yellow
    Write-Host "      💡 Usando dados públicos disponíveis" -ForegroundColor Cyan
}

Write-Host ""

if ($apis_available.Count -eq 0) {
    Write-Host "⚠️ Nenhuma API configurada!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "💡 Para configurar APIs, adicione ao .env:" -ForegroundColor Cyan
    Write-Host "   BAYUT_API_KEY=sua_chave_aqui" -ForegroundColor White
    Write-Host "   PROPERTYFINDER_CLIENT_ID=seu_client_id" -ForegroundColor White
    Write-Host "   PROPERTYFINDER_CLIENT_SECRET=seu_client_secret" -ForegroundColor White
    Write-Host "   DLD_API_KEY=sua_chave_dld" -ForegroundColor White
    Write-Host ""
    Write-Host "📖 Consulte .env.apis para instruções completas" -ForegroundColor Cyan
    Write-Host ""
}

# Check MongoDB Atlas
if (-not [Environment]::GetEnvironmentVariable("MONGO_ATLAS_URI", "Process")) {
    Write-Host "❌ MONGO_ATLAS_URI não configurada!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ MongoDB Atlas configurado" -ForegroundColor Green
Write-Host ""

if ($DryRun) {
    Write-Host "🔍 Modo DRY RUN - Nenhum dado será salvo" -ForegroundColor Yellow
    Write-Host ""
}

# Build if requested
if ($Build -or $Release) {
    Write-Host "🔨 Compilando..." -ForegroundColor Yellow
    
    if ($Release) {
        cargo build --release --bin fetch-dubai-data
        $binary_path = "target/release/fetch-dubai-data.exe"
    } else {
        cargo build --bin fetch-dubai-data
        $binary_path = "target/debug/fetch-dubai-data.exe"
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Erro ao compilar" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ Compilação concluída" -ForegroundColor Green
    Write-Host ""
} else {
    $binary_path = "target/debug/fetch-dubai-data.exe"
    if (-not (Test-Path $binary_path)) {
        $binary_path = "target/release/fetch-dubai-data.exe"
        if (-not (Test-Path $binary_path)) {
            Write-Host "❌ Binário não encontrado!" -ForegroundColor Red
            Write-Host "💡 Execute com -Build para compilar primeiro" -ForegroundColor Yellow
            exit 1
        }
    }
}

# Run the fetcher
Write-Host "🚀 Iniciando fetch de dados..." -ForegroundColor Cyan
Write-Host ""

& $binary_path

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Fetch concluído!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📊 Próximos passos:" -ForegroundColor Cyan
    Write-Host "   1. Verifique os dados no MongoDB Atlas" -ForegroundColor White
    Write-Host "   2. Execute .\sync-atlas.ps1 para atualizar o site" -ForegroundColor White
    Write-Host "   3. Configure mais APIs em .env para dados completos" -ForegroundColor White
} else {
    Write-Host ""
    Write-Host "❌ Erro no fetch" -ForegroundColor Red
    exit 1
}
