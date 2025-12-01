#!/bin/bash
# Deploy Script - Dubai Project no Servidor Arxis

set -e

echo "🚀 Deploy Dubai Project - Servidor Arxis"
echo "========================================"

# Configurações
REMOTE_USER="arxis"
REMOTE_HOST="arxis.io"
REMOTE_PATH="/opt/arxis/dubai"
BACKUP_PATH="/opt/arxis/backups/dubai"

# Verificar se estamos no diretório correto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Erro: Execute este script no diretório raiz do projeto"
    exit 1
fi

# 1. Build local
echo ""
echo "📦 Building projeto em modo release..."
cargo build --release

# 2. Executar testes
echo ""
echo "🧪 Executando testes..."
cargo test --release --all

# 3. Criar backup no servidor
echo ""
echo "💾 Criando backup no servidor..."
ssh ${REMOTE_USER}@${REMOTE_HOST} "mkdir -p ${BACKUP_PATH}"
ssh ${REMOTE_USER}@${REMOTE_HOST} "
    if [ -d ${REMOTE_PATH} ]; then
        BACKUP_NAME=backup_\$(date +%Y%m%d_%H%M%S).tar.gz
        cd ${REMOTE_PATH}/..
        tar -czf ${BACKUP_PATH}/\${BACKUP_NAME} dubai/
        echo '✅ Backup criado: \${BACKUP_NAME}'
    fi
"

# 4. Criar diretórios no servidor
echo ""
echo "📁 Preparando diretórios no servidor..."
ssh ${REMOTE_USER}@${REMOTE_HOST} "
    mkdir -p ${REMOTE_PATH}
    mkdir -p /var/lib/arxis/dubai/{data,backups}
    mkdir -p /var/log/arxis/dubai
    mkdir -p /etc/arxis/certs
"

# 5. Copiar arquivos
echo ""
echo "📤 Copiando arquivos para servidor..."

# Copiar binário
scp target/release/dubai-project ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/

# Copiar Docker files
scp Dockerfile docker-compose.yml ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/

# Copiar configurações
scp .env.production ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/.env

# Copiar scripts de monitoramento
scp -r monitoring ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/

# 6. Build e deploy com Docker
echo ""
echo "🐳 Construindo e iniciando containers..."
ssh ${REMOTE_USER}@${REMOTE_HOST} "
    cd ${REMOTE_PATH}

    # Build da imagem
    docker-compose build

    # Parar containers antigos
    docker-compose down

    # Iniciar novos containers
    docker-compose up -d

    # Verificar status
    sleep 5
    docker-compose ps
"

# 7. Verificar health
echo ""
echo "🏥 Verificando health do serviço..."
sleep 10

HEALTH_STATUS=\$(ssh ${REMOTE_USER}@${REMOTE_HOST} "curl -s http://localhost:8080/health || echo 'FAILED'")

if [ "\$HEALTH_STATUS" = "FAILED" ]; then
    echo "❌ Health check falhou!"
    echo "📋 Logs:"
    ssh ${REMOTE_USER}@${REMOTE_HOST} "docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs --tail=50 dubai-project"
    exit 1
else
    echo "✅ Health check passou!"
fi

# 8. Mostrar status
echo ""
echo "📊 Status do deploy:"
ssh ${REMOTE_USER}@${REMOTE_HOST} "
    echo '=== Containers ==='
    docker-compose -f ${REMOTE_PATH}/docker-compose.yml ps

    echo ''
    echo '=== Últimas logs ==='
    docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs --tail=20 dubai-project
"

echo ""
echo "✅ Deploy concluído com sucesso!"
echo ""
echo "🌐 URLs:"
echo "   API:      https://dubai.arxis.io:8080"
echo "   Métricas: http://dubai.arxis.io:9090"
echo "   Grafana:  http://dubai.arxis.io:3000"
echo ""
echo "📊 Comandos úteis:"
echo "   ssh ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs -f dubai-project'"
echo "   ssh ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml restart dubai-project'"
echo "   ssh ${REMOTE_USER}@${REMOTE_HOST} 'docker-compose -f ${REMOTE_PATH}/docker-compose.yml ps'"
