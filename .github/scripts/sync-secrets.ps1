# ==========================================
# GitHub Secrets Sync Script
# ==========================================
# Sincroniza todas as variáveis de ambiente do .env para GitHub Actions Secrets
# Requer: GitHub CLI (gh) autenticado
# Uso: Carregue o .env primeiro, depois execute .\sync-secrets.ps1
#      Get-Content .env | ForEach-Object { if ($_ -match '^([^#=]+)=(.+)$') { [Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim(), 'Process') } }
#      .\.github\scripts\sync-secrets.ps1

$ErrorActionPreference = "Stop"

# Verifica se gh CLI está instalado
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Error "GitHub CLI (gh) não encontrado. Instale com: winget install GitHub.cli"
    exit 1
}

# Verifica autenticação
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Autenticação necessária. Executando gh auth login..."
    gh auth login
}

$repo = "avilaops/Dubai"
Write-Host "[SYNC] Sincronizando secrets para $repo..." -ForegroundColor Cyan

# Função auxiliar para criar secret
function Set-GitHubSecret {
    param(
        [string]$Name,
        [string]$Value
    )

    if ([string]::IsNullOrWhiteSpace($Value)) {
        Write-Host "  [SKIP] $Name (vazio)" -ForegroundColor Yellow
        return
    }

    try {
        $Value | gh secret set $Name -R $repo
        Write-Host "  [OK] $Name" -ForegroundColor Green
    }
    catch {
        Write-Host "  [ERROR] $Name - $_" -ForegroundColor Red
    }
}

Write-Host "`n[DNS & Domain Services]" -ForegroundColor Magenta
Set-GitHubSecret "PORKBUN_API_KEY" "$env:PORKBUN_API_KEY"
Set-GitHubSecret "PORKBUN_SECRET_KEY" "$env:PORKBUN_SECRET_KEY"
Set-GitHubSecret "CLOUDFLARE_API_KEY" "$env:CLOUDFLARE_API_KEY"

Write-Host "`n[Database]" -ForegroundColor Magenta
Set-GitHubSecret "MONGO_ATLAS_URI" "$env:MONGO_ATLAS_URI"

Write-Host "`n[Payment Services]" -ForegroundColor Magenta
Set-GitHubSecret "PAYPAL_ID" "$env:PAYPAL_ID"
Set-GitHubSecret "PAYPAL_TOKEN_API" "$env:PAYPAL_TOKEN_API"
Set-GitHubSecret "STRIPE_API" "$env:STRIPE_API"

Write-Host "`n[AI & ML Services]" -ForegroundColor Magenta
Set-GitHubSecret "OPENAI_API_KEY" "$env:OPENAI_API_KEY"
Set-GitHubSecret "LANGSMITH_API_KEY" "$env:LANGSMITH_API_KEY"
Set-GitHubSecret "HF_TOKEN" "$env:HF_TOKEN"

Write-Host "`n[Developer Tools]" -ForegroundColor Magenta
# GitHub secrets cannot start with GITHUB_ prefix - using ALT_ prefix
Set-GitHubSecret "ALT_GITHUB_USERNAME" "$env:GITHUB_USERNAME"
Set-GitHubSecret "ALT_GITHUB_TOKEN" "$env:GITHUB_TOKEN"
Set-GitHubSecret "CARGO_REGISTRY_TOKEN" "$env:CARGO_REGISTRY_TOKEN"
Set-GitHubSecret "SENTRY_TOKEN_API" "$env:SENTRY_TOKEN_API"
Set-GitHubSecret "NGROK" "$env:NGROK"

Write-Host "`n[Google Cloud Services]" -ForegroundColor Magenta
Set-GitHubSecret "GCLOUD_API_TOKEN" "$env:GCLOUD_API_TOKEN"
Set-GitHubSecret "GCLOUD_CLIENT" "$env:GCLOUD_CLIENT"
Set-GitHubSecret "GCLOUD_SECRET_KEY" "$env:GCLOUD_SECRET_KEY"
Set-GitHubSecret "GCLOUD_MAPS_ID" "$env:GCLOUD_MAPS_ID"

Write-Host "`n[Other Services]" -ForegroundColor Magenta
Set-GitHubSecret "GRAVATAR" "$env:GRAVATAR"
Set-GitHubSecret "IMAZING" "$env:IMAZING"

Write-Host "`n[Gmail Accounts]" -ForegroundColor Magenta
Set-GitHubSecret "GMAIL_USER_AVILACARGASRAPIDAS" "$env:GMAIL_USER_AVILACARGASRAPIDAS"
Set-GitHubSecret "GMAIL_PASSWORD_APP_AVILACARGASRAPIDAS" "$env:GMAIL_PASSWORD_APP_AVILACARGASRAPIDAS"
Set-GitHubSecret "GMAIL_USER_NICOLASROSAAB" "$env:GMAIL_USER_NICOLASROSAAB"
Set-GitHubSecret "GMAIL_PASSWORD_APP_NICOLASROSAAB" "$env:GMAIL_PASSWORD_APP_NICOLASROSAAB"
Set-GitHubSecret "GMAIL_USER_FATURAMENTO" "$env:GMAIL_USER_FATURAMENTO"
Set-GitHubSecret "GMAIL_PASSWORD_APP_FATURAMENTO" "$env:GMAIL_PASSWORD_APP_FATURAMENTO"

Write-Host "`n[SUCCESS] Sincronizacao completa!" -ForegroundColor Green
Write-Host "Verifique em: https://github.com/$repo/settings/secrets/actions" -ForegroundColor Cyan
