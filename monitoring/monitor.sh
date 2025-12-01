#!/bin/bash
# Script de Monitoramento - Dubai Project

set -e

echo "📊 Monitoramento Dubai Project"
echo "=============================="

# Cores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Funções de monitoramento

check_service() {
    echo -e "\n${YELLOW}🔍 Status do Serviço${NC}"

    if systemctl is-active --quiet dubai-project; then
        echo -e "${GREEN}✅ Serviço está RODANDO${NC}"
    else
        echo -e "${RED}❌ Serviço está PARADO${NC}"
        return 1
    fi
}

check_health() {
    echo -e "\n${YELLOW}🏥 Health Check${NC}"

    HEALTH=$(curl -s http://localhost:8080/health || echo "FAILED")

    if [ "$HEALTH" != "FAILED" ]; then
        echo -e "${GREEN}✅ Health check: OK${NC}"
        echo "$HEALTH"
    else
        echo -e "${RED}❌ Health check: FAILED${NC}"
        return 1
    fi
}

check_resources() {
    echo -e "\n${YELLOW}💻 Uso de Recursos${NC}"

    # CPU
    CPU=$(ps aux | grep dubai-project | grep -v grep | awk '{print $3}')
    echo "CPU: ${CPU}%"

    # Memória
    MEM=$(ps aux | grep dubai-project | grep -v grep | awk '{print $4}')
    MEM_MB=$(ps aux | grep dubai-project | grep -v grep | awk '{print $6/1024}')
    echo "Memória: ${MEM}% (${MEM_MB} MB)"

    # Disco
    DISK=$(df -h /var/lib/arxis/dubai | awk 'NR==2 {print $5}')
    echo "Disco: ${DISK}"

    # Alertas
    if (( $(echo "$CPU > 80" | bc -l) )); then
        echo -e "${RED}⚠️  CPU alta!${NC}"
    fi

    if (( $(echo "$MEM > 80" | bc -l) )); then
        echo -e "${RED}⚠️  Memória alta!${NC}"
    fi
}

check_logs() {
    echo -e "\n${YELLOW}📋 Últimos Logs (erros)${NC}"

    journalctl -u dubai-project -n 20 --no-pager | grep -i "error\|warn\|fail" || echo "Sem erros recentes"
}

check_database() {
    echo -e "\n${YELLOW}💾 Banco de Dados${NC}"

    DB_SIZE=$(du -sh /var/lib/arxis/dubai/data 2>/dev/null | cut -f1)
    echo "Tamanho: ${DB_SIZE}"

    BACKUP_COUNT=$(ls -1 /var/lib/arxis/dubai/backups/*.db 2>/dev/null | wc -l)
    echo "Backups: ${BACKUP_COUNT}"

    if [ "$BACKUP_COUNT" -eq 0 ]; then
        echo -e "${YELLOW}⚠️  Nenhum backup encontrado${NC}"
    fi
}

check_network() {
    echo -e "\n${YELLOW}🌐 Conectividade${NC}"

    # Porta 8080
    if netstat -tuln | grep -q ":8080"; then
        echo -e "${GREEN}✅ Porta 8080: ABERTA${NC}"
    else
        echo -e "${RED}❌ Porta 8080: FECHADA${NC}"
    fi

    # Porta 9090 (métricas)
    if netstat -tuln | grep -q ":9090"; then
        echo -e "${GREEN}✅ Porta 9090: ABERTA${NC}"
    else
        echo -e "${RED}❌ Porta 9090: FECHADA${NC}"
    fi

    # Teste de conectividade externa
    if curl -s --connect-timeout 5 https://api.bayut.com > /dev/null; then
        echo -e "${GREEN}✅ Bayut API: ACESSÍVEL${NC}"
    else
        echo -e "${RED}❌ Bayut API: INACESSÍVEL${NC}"
    fi
}

check_metrics() {
    echo -e "\n${YELLOW}📈 Métricas Prometheus${NC}"

    METRICS=$(curl -s http://localhost:9090/metrics 2>/dev/null || echo "FAILED")

    if [ "$METRICS" != "FAILED" ]; then
        # Extrair algumas métricas importantes
        echo "Requisições HTTP:"
        echo "$METRICS" | grep "^http_requests_total" | head -3

        echo ""
        echo "Cache:"
        echo "$METRICS" | grep "^cache_" | head -3
    else
        echo -e "${RED}❌ Não foi possível obter métricas${NC}"
    fi
}

check_ssl() {
    echo -e "\n${YELLOW}🔒 Certificados SSL${NC}"

    CERT_PATH="/etc/letsencrypt/live/dubai.arxis.io/cert.pem"

    if [ -f "$CERT_PATH" ]; then
        EXPIRY=$(openssl x509 -enddate -noout -in "$CERT_PATH" | cut -d= -f2)
        DAYS_LEFT=$(( ($(date -d "$EXPIRY" +%s) - $(date +%s)) / 86400 ))

        echo "Expira em: $EXPIRY"
        echo "Dias restantes: $DAYS_LEFT"

        if [ "$DAYS_LEFT" -lt 30 ]; then
            echo -e "${YELLOW}⚠️  Certificado expira em menos de 30 dias!${NC}"
        else
            echo -e "${GREEN}✅ Certificado válido${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  Certificado Let's Encrypt não encontrado${NC}"
    fi
}

# Executar todas as verificações
check_service
check_health
check_resources
check_logs
check_database
check_network
check_metrics
check_ssl

# Resumo final
echo -e "\n${GREEN}=============================="
echo "✅ Monitoramento concluído"
echo "==============================${NC}"
