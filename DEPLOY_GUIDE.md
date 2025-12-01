# Guia Completo de Deploy - Dubai Project no Servidor Arxis

## 📋 Pré-requisitos

### No Servidor Arxis
- Docker e Docker Compose instalados
- Nginx instalado
- Certbot para SSL (Let's Encrypt)
- Usuário `arxis` criado
- SSH configurado

### Localmente
- Rust 1.75+ instalado
- Git configurado
- Acesso SSH ao servidor (chave pública configurada)

## 🚀 Deploy Completo - Passo a Passo

### 1. Configurar API Keys

Editar `.env.production` com suas credenciais:

```bash
# Bayut
BAYUT_API_KEY=your_bayut_key
BAYUT_API_SECRET=your_bayut_secret

# PropertyFinder
PROPERTYFINDER_CLIENT_ID=your_pf_client_id
PROPERTYFINDER_CLIENT_SECRET=your_pf_secret

# Dubai Land Department
DLD_API_KEY=your_dld_key

# Outros...
```

### 2. Build e Test Local

```powershell
# Build
cargo build --release

# Testes
cargo test --release --all

# Executar localmente para validar
cargo run --release
```

### 3. Deploy Automático

#### Opção A: PowerShell (Windows)
```powershell
.\deploy.ps1
```

#### Opção B: Bash (Linux/Mac)
```bash
chmod +x deploy.sh
./deploy.sh
```

### 4. Configurar Nginx (Primeira vez)

No servidor:

```bash
# Copiar configuração
sudo cp /opt/arxis/dubai/deploy/nginx-dubai.conf /etc/nginx/sites-available/dubai.arxis.io

# Ativar site
sudo ln -s /etc/nginx/sites-available/dubai.arxis.io /etc/nginx/sites-enabled/

# Testar configuração
sudo nginx -t

# Recarregar Nginx
sudo systemctl reload nginx
```

### 5. Configurar SSL/TLS

```bash
# Instalar Certbot
sudo apt install certbot python3-certbot-nginx

# Gerar certificado
sudo certbot --nginx -d dubai.arxis.io

# Renovação automática já está configurada
```

### 6. Configurar systemd (Alternativa ao Docker)

Se preferir rodar sem Docker:

```bash
# Copiar service file
sudo cp /opt/arxis/dubai/deploy/dubai-project.service /etc/systemd/system/

# Recarregar systemd
sudo systemctl daemon-reload

# Habilitar e iniciar
sudo systemctl enable dubai-project
sudo systemctl start dubai-project

# Verificar status
sudo systemctl status dubai-project
```

### 7. Verificar Deploy

```bash
# Health check
curl https://dubai.arxis.io/health

# Verificar logs
ssh arxis@arxis.io 'docker-compose -f /opt/arxis/dubai/docker-compose.yml logs -f dubai-project'

# Ou com systemd
ssh arxis@arxis.io 'journalctl -u dubai-project -f'
```

## 📊 Monitoramento

### Acessar Dashboards

- **API**: https://dubai.arxis.io
- **Métricas**: http://dubai.arxis.io:9090/metrics
- **Prometheus**: http://dubai.arxis.io:9091
- **Grafana**: http://grafana.arxis.io:3000

### Script de Monitoramento

```bash
# No servidor
cd /opt/arxis/dubai/monitoring
chmod +x monitor.sh
./monitor.sh
```

### Comandos Úteis

```bash
# Status dos containers
docker-compose -f /opt/arxis/dubai/docker-compose.yml ps

# Logs em tempo real
docker-compose -f /opt/arxis/dubai/docker-compose.yml logs -f

# Restart do serviço
docker-compose -f /opt/arxis/dubai/docker-compose.yml restart dubai-project

# Parar tudo
docker-compose -f /opt/arxis/dubai/docker-compose.yml down

# Iniciar tudo
docker-compose -f /opt/arxis/dubai/docker-compose.yml up -d

# Ver recursos
docker stats
```

## 🔄 Atualização (Deploy Novo)

```bash
# Localmente
git pull
cargo build --release
cargo test --release --all

# Deploy
.\deploy.ps1  # ou ./deploy.sh

# Deploy cria backup automático antes de atualizar
```

## 🔧 Troubleshooting

### Serviço não inicia

```bash
# Ver logs
docker-compose logs dubai-project

# Ou systemd
journalctl -u dubai-project -n 100

# Verificar portas
sudo netstat -tulpn | grep -E '8080|9090'
```

### Health check falha

```bash
# Testar diretamente
curl http://localhost:8080/health

# Ver logs de erro
docker-compose logs dubai-project | grep -i error
```

### SSL não funciona

```bash
# Verificar certificado
sudo certbot certificates

# Renovar forçado
sudo certbot renew --force-renewal

# Testar configuração nginx
sudo nginx -t

# Ver logs nginx
sudo tail -f /var/log/nginx/dubai.arxis.io-error.log
```

### Banco de dados corrompido

```bash
# Restaurar backup mais recente
cd /var/lib/arxis/dubai/backups
ls -lht  # Ver backups

# Copiar backup
cp backup_YYYYMMDD_HHMMSS.db ../data/aviladb.db

# Reiniciar serviço
docker-compose restart dubai-project
```

## 📈 Performance

### Otimizações Configuradas

- **HTTP/2** habilitado no Nginx
- **Keepalive** de conexões (32 conexões)
- **Gzip** para respostas textuais
- **Cache** de 10.000 items (1h TTL)
- **Rate limiting** (60 req/min por IP)
- **Workers** configurados: 4 para aplicação, 8 para jobs

### Limites de Recursos (Docker)

- **CPU**: 2 cores max, 1 core reservado
- **RAM**: 1GB max, 512MB reservado

## 🔐 Segurança

### Checklist

- ✅ HTTPS obrigatório (redirect de HTTP)
- ✅ TLS 1.2+ apenas
- ✅ Headers de segurança (HSTS, X-Frame-Options, etc.)
- ✅ Usuário não-root nos containers
- ✅ Secrets em `.env` (não commitado)
- ✅ Rate limiting configurado
- ✅ Métricas protegidas (IPs internos apenas)

## 📞 Suporte

Se algo der errado:

1. Verificar logs: `./monitoring/monitor.sh`
2. Testar health: `curl https://dubai.arxis.io/health`
3. Ver métricas: `http://dubai.arxis.io:9091`
4. Grafana: `http://grafana.arxis.io:3000`

## 🎯 Próximos Passos

1. ✅ Deploy inicial
2. Configurar alertas por email/webhook
3. Configurar backups automáticos diários
4. Adicionar CI/CD com GitHub Actions
5. Configurar multi-region (se necessário)
