#!/usr/bin/env pwsh
# Script para rodar localmente com Docker - Dubai Project

$ErrorActionPreference = "Stop"

Write-Host "🚀 Iniciando Dubai Project - LOCAL" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green

# Verificar se Docker está rodando
Write-Host "`n🐳 Verificando Docker..." -ForegroundColor Cyan
$dockerRunning = docker info 2>$null
if (-not $dockerRunning) {
    Write-Host "❌ Docker não está rodando! Inicie o Docker Desktop primeiro." -ForegroundColor Red
    exit 1
}
Write-Host "✅ Docker está rodando" -ForegroundColor Green

# Parar containers antigos se existirem
Write-Host "`n🛑 Parando containers antigos..." -ForegroundColor Cyan
docker-compose down 2>$null

# Build da aplicação
Write-Host "`n📦 Building aplicação..." -ForegroundColor Cyan
cargo build --release

# Iniciar containers
Write-Host "`n🐳 Iniciando containers (Dubai + Prometheus + Grafana)..." -ForegroundColor Cyan
docker-compose up -d

# Aguardar inicialização
Write-Host "`n⏳ Aguardando serviços iniciarem..." -ForegroundColor Cyan
Start-Sleep -Seconds 10

# Health check
Write-Host "`n🏥 Verificando health..." -ForegroundColor Cyan
$health = curl.exe -s http://localhost:8080/health 2>$null
if ($health) {
    Write-Host "✅ Serviço está UP!" -ForegroundColor Green
} else {
    Write-Host "⚠️  Health check falhou, mas container pode estar iniciando..." -ForegroundColor Yellow
}

# Status dos containers
Write-Host "`n📊 Status dos containers:" -ForegroundColor Cyan
docker-compose ps

# Mostrar logs
Write-Host "`n📋 Últimas logs:" -ForegroundColor Cyan
docker-compose logs --tail=30 dubai-project

Write-Host "`n✅ Dubai Project está rodando!" -ForegroundColor Green
Write-Host "`n🌐 Acesse:" -ForegroundColor Cyan
Write-Host "   API:        http://localhost:8080" -ForegroundColor White
Write-Host "   Health:     http://localhost:8080/health" -ForegroundColor White
Write-Host "   Prometheus: http://localhost:9091" -ForegroundColor White
Write-Host "   Grafana:    http://localhost:3000 (admin/admin)" -ForegroundColor White

Write-Host "`n📊 Comandos úteis:" -ForegroundColor Cyan
Write-Host "   docker-compose logs -f              # Ver logs em tempo real" -ForegroundColor Gray
Write-Host "   docker-compose ps                   # Status dos containers" -ForegroundColor Gray
Write-Host "   docker-compose restart dubai-project # Restart da aplicação" -ForegroundColor Gray
Write-Host "   docker-compose down                 # Parar tudo" -ForegroundColor Gray
