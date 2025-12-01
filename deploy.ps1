# Deploy Script PowerShell - Dubai Project no Servidor Arxis

$ErrorActionPreference = "Stop"

Write-Host "🚀 Deploy Dubai Project - Servidor Arxis" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# Configurações
$REMOTE_USER = "arxis"
$REMOTE_HOST = "arxis.io"
$REMOTE_PATH = "/opt/arxis/dubai"
$SSH_KEY = "$env:USERPROFILE\.ssh\id_rsa"

# Verificar se estamos no diretório correto
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Erro: Execute este script no diretório raiz do projeto" -ForegroundColor Red
    exit 1
}

# 1. Build local
Write-Host "`n📦 Building projeto em modo release..." -ForegroundColor Cyan
cargo build --release

# 2. Executar testes
Write-Host "`n🧪 Executando testes..." -ForegroundColor Cyan
cargo test --release --all

# 3. Criar backup no servidor
Write-Host "`n💾 Criando backup no servidor..." -ForegroundColor Cyan
ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
    mkdir -p /opt/arxis/backups/dubai
    if [ -d $REMOTE_PATH ]; then
        BACKUP_NAME=backup_`$(date +%Y%m%d_%H%M%S).tar.gz
        cd ${REMOTE_PATH}/..
        tar -czf /opt/arxis/backups/dubai/`${BACKUP_NAME} dubai/
        echo '✅ Backup criado: `${BACKUP_NAME}'
    fi
"@

# 4. Copiar arquivos via SCP
Write-Host "`n📤 Copiando arquivos para servidor..." -ForegroundColor Cyan

# Criar estrutura de diretórios
ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
    mkdir -p $REMOTE_PATH
    mkdir -p /var/lib/arxis/dubai/{data,backups}
    mkdir -p /var/log/arxis/dubai
    mkdir -p /etc/arxis/certs
"@

# Copiar arquivos
scp -i $SSH_KEY target/release/dubai-project.exe ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/dubai-project
scp -i $SSH_KEY Dockerfile ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/
scp -i $SSH_KEY docker-compose.yml ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/
scp -i $SSH_KEY .env.production ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/.env
scp -i $SSH_KEY -r monitoring ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/

# 5. Deploy com Docker
Write-Host "`n🐳 Construindo e iniciando containers..." -ForegroundColor Cyan
ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
    cd $REMOTE_PATH
    docker-compose build
    docker-compose down
    docker-compose up -d
    sleep 5
    docker-compose ps
"@

# 6. Health check
Write-Host "`n🏥 Verificando health do serviço..." -ForegroundColor Cyan
Start-Sleep -Seconds 10

$healthCheck = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "curl -s http://localhost:8080/health || echo 'FAILED'"

if ($healthCheck -eq "FAILED") {
    Write-Host "❌ Health check falhou!" -ForegroundColor Red
    Write-Host "📋 Logs:" -ForegroundColor Yellow
    ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs --tail=50 dubai-project"
    exit 1
}

Write-Host "✅ Health check passou!" -ForegroundColor Green

# 7. Status final
Write-Host "`n📊 Status do deploy:" -ForegroundColor Cyan
ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
    echo '=== Containers ==='
    docker-compose -f ${REMOTE_PATH}/docker-compose.yml ps

    echo ''
    echo '=== Últimas logs ==='
    docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs --tail=20 dubai-project
"@

Write-Host "`n✅ Deploy concluído com sucesso!" -ForegroundColor Green
Write-Host "`n🌐 URLs:" -ForegroundColor Cyan
Write-Host "   API:      https://dubai.arxis.io:8080"
Write-Host "   Métricas: http://dubai.arxis.io:9090"
Write-Host "   Grafana:  http://dubai.arxis.io:3000"
Write-Host "`n📊 Comandos úteis:" -ForegroundColor Cyan
Write-Host "   ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs -f dubai-project'"
Write-Host "   ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml restart dubai-project'"
Write-Host "   ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml ps'"
