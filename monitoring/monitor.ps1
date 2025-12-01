#!/usr/bin/env pwsh
# Script de Monitoramento PowerShell - Dubai Project

$ErrorActionPreference = "Stop"

Write-Host "📊 Monitoramento Dubai Project" -ForegroundColor Cyan
Write-Host "==============================" -ForegroundColor Cyan

# Configurações
$REMOTE_USER = "arxis"
$REMOTE_HOST = "arxis.io"
$SSH_KEY = "$env:USERPROFILE\.ssh\id_rsa"

function Check-Service {
    Write-Host "`n🔍 Status do Serviço" -ForegroundColor Yellow

    $status = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "systemctl is-active dubai-project 2>/dev/null || echo 'NOT_INSTALLED'"

    if ($status -eq "active") {
        Write-Host "✅ Serviço está RODANDO" -ForegroundColor Green
        return $true
    } elseif ($status -eq "NOT_INSTALLED") {
        Write-Host "⚠️  Serviço systemd não configurado (usando Docker)" -ForegroundColor Yellow
        return $true
    } else {
        Write-Host "❌ Serviço está PARADO" -ForegroundColor Red
        return $false
    }
}

function Check-Docker {
    Write-Host "`n🐳 Status Docker" -ForegroundColor Yellow

    $containers = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "docker-compose -f /opt/arxis/dubai/docker-compose.yml ps 2>/dev/null"

    Write-Host $containers
}

function Check-Health {
    Write-Host "`n🏥 Health Check" -ForegroundColor Yellow

    $health = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "curl -s http://localhost:8080/health 2>/dev/null || echo 'FAILED'"

    if ($health -ne "FAILED") {
        Write-Host "✅ Health check: OK" -ForegroundColor Green
        Write-Host $health
        return $true
    } else {
        Write-Host "❌ Health check: FAILED" -ForegroundColor Red
        return $false
    }
}

function Check-Resources {
    Write-Host "`n💻 Uso de Recursos" -ForegroundColor Yellow

    $resources = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
ps aux | grep -E 'dubai-project|docker.*dubai' | grep -v grep | awk '{printf "PID: %s, CPU: %s%%, MEM: %s%%, RSS: %s KB\n", `$2, `$3, `$4, `$6}'
"@

    if ($resources) {
        Write-Host $resources
    } else {
        Write-Host "⚠️  Processo não encontrado" -ForegroundColor Yellow
    }

    # Uso de disco
    $disk = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "df -h /var/lib/arxis/dubai 2>/dev/null | awk 'NR==2 {print `$5}'"
    Write-Host "Disco: $disk"
}

function Check-Logs {
    Write-Host "`n📋 Últimos Logs (erros/warnings)" -ForegroundColor Yellow

    $logs = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
docker-compose -f /opt/arxis/dubai/docker-compose.yml logs --tail=20 dubai-project 2>/dev/null | grep -iE 'error|warn|fail' || echo 'Sem erros recentes'
"@

    Write-Host $logs
}

function Check-Database {
    Write-Host "`n💾 Banco de Dados" -ForegroundColor Yellow

    $dbInfo = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
du -sh /var/lib/arxis/dubai/data 2>/dev/null | cut -f1
ls -1 /var/lib/arxis/dubai/backups/*.db 2>/dev/null | wc -l
"@

    $lines = $dbInfo -split "`n"
    Write-Host "Tamanho: $($lines[0])"
    Write-Host "Backups: $($lines[1])"

    if ($lines[1] -eq "0") {
        Write-Host "⚠️  Nenhum backup encontrado" -ForegroundColor Yellow
    }
}

function Check-Network {
    Write-Host "`n🌐 Conectividade" -ForegroundColor Yellow

    $ports = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
netstat -tuln 2>/dev/null | grep -E ':8080|:9090' || ss -tuln | grep -E ':8080|:9090'
"@

    if ($ports -match ":8080") {
        Write-Host "✅ Porta 8080: ABERTA" -ForegroundColor Green
    } else {
        Write-Host "❌ Porta 8080: FECHADA" -ForegroundColor Red
    }

    if ($ports -match ":9090") {
        Write-Host "✅ Porta 9090: ABERTA" -ForegroundColor Green
    } else {
        Write-Host "❌ Porta 9090: FECHADA" -ForegroundColor Red
    }

    # Teste de conectividade externa
    $bayut = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} "curl -s --connect-timeout 5 https://api.bayut.com > /dev/null 2>&1 && echo 'OK' || echo 'FAILED'"

    if ($bayut -eq "OK") {
        Write-Host "✅ Bayut API: ACESSÍVEL" -ForegroundColor Green
    } else {
        Write-Host "❌ Bayut API: INACESSÍVEL" -ForegroundColor Red
    }
}

function Check-SSL {
    Write-Host "`n🔒 Certificados SSL" -ForegroundColor Yellow

    $certInfo = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
if [ -f /etc/letsencrypt/live/dubai.arxis.io/cert.pem ]; then
    EXPIRY=`$(openssl x509 -enddate -noout -in /etc/letsencrypt/live/dubai.arxis.io/cert.pem | cut -d= -f2)
    DAYS_LEFT=`$(( (`$(date -d "`$EXPIRY" +%s) - `$(date +%s)) / 86400 ))
    echo "Expira em: `$EXPIRY"
    echo "Dias restantes: `$DAYS_LEFT"
else
    echo "NOT_FOUND"
fi
"@

    if ($certInfo -ne "NOT_FOUND") {
        Write-Host $certInfo

        if ($certInfo -match "Dias restantes: (\d+)") {
            $days = [int]$matches[1]
            if ($days -lt 30) {
                Write-Host "⚠️  Certificado expira em menos de 30 dias!" -ForegroundColor Yellow
            } else {
                Write-Host "✅ Certificado válido" -ForegroundColor Green
            }
        }
    } else {
        Write-Host "⚠️  Certificado Let's Encrypt não encontrado" -ForegroundColor Yellow
    }
}

function Check-Metrics {
    Write-Host "`n📈 Métricas Prometheus" -ForegroundColor Yellow

    $metrics = ssh -i $SSH_KEY ${REMOTE_USER}@${REMOTE_HOST} @"
curl -s http://localhost:9090/metrics 2>/dev/null | grep -E '^http_requests_total|^cache_' | head -6 || echo 'FAILED'
"@

    if ($metrics -ne "FAILED") {
        Write-Host $metrics
    } else {
        Write-Host "❌ Não foi possível obter métricas" -ForegroundColor Red
    }
}

# Executar todas as verificações
try {
    Check-Service
    Check-Docker
    Check-Health
    Check-Resources
    Check-Logs
    Check-Database
    Check-Network
    Check-SSL
    Check-Metrics

    Write-Host "`n==============================" -ForegroundColor Green
    Write-Host "✅ Monitoramento concluído" -ForegroundColor Green
    Write-Host "==============================" -ForegroundColor Green

} catch {
    Write-Host "`n❌ Erro durante monitoramento: $_" -ForegroundColor Red
    exit 1
}
