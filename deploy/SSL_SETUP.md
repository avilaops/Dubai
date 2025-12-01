# Configuração de SSL/TLS - Dubai Project

## Certificados Let's Encrypt

### 1. Instalar Certbot
```bash
sudo apt update
sudo apt install certbot python3-certbot-nginx
```

### 2. Gerar Certificado
```bash
# Para dubai.arxis.io
sudo certbot --nginx -d dubai.arxis.io

# Para grafana (se em subdomínio separado)
sudo certbot --nginx -d grafana.arxis.io
```

### 3. Renovação Automática
Certbot cria um cron job automaticamente para renovar. Verificar:
```bash
sudo systemctl status certbot.timer
```

Testar renovação:
```bash
sudo certbot renew --dry-run
```

### 4. Localizações dos Certificados
```
/etc/letsencrypt/live/dubai.arxis.io/fullchain.pem
/etc/letsencrypt/live/dubai.arxis.io/privkey.pem
/etc/letsencrypt/live/dubai.arxis.io/chain.pem
/etc/letsencrypt/live/dubai.arxis.io/cert.pem
```

## Certificados Auto-assinados (Desenvolvimento)

### Gerar certificado auto-assinado
```bash
sudo mkdir -p /etc/arxis/certs
cd /etc/arxis/certs

# Gerar chave privada
sudo openssl genrsa -out dubai.arxis.io.key 2048

# Gerar CSR
sudo openssl req -new -key dubai.arxis.io.key -out dubai.arxis.io.csr \
  -subj "/C=AE/ST=Dubai/L=Dubai/O=Arxis/CN=dubai.arxis.io"

# Gerar certificado (válido por 1 ano)
sudo openssl x509 -req -days 365 -in dubai.arxis.io.csr \
  -signkey dubai.arxis.io.key -out dubai.arxis.io.crt

# Permissões
sudo chmod 600 dubai.arxis.io.key
sudo chmod 644 dubai.arxis.io.crt
```

## Configuração TLS no Código

O Dubai Project usa TLS nativo (ver `crates/avila-http/src/tls.rs`):

```rust
// TLS 1.2 com SNI
let tls = TlsStream::connect("api.bayut.com:443", "api.bayut.com")?;

// Cipher suites suportados
- TLS_RSA_WITH_AES_128_GCM_SHA256 (0x009C)
- TLS_RSA_WITH_AES_128_CBC_SHA (0x002F)
- TLS_RSA_WITH_AES_256_CBC_SHA (0x0035)
```

## Verificação

### Testar conexão TLS
```bash
# Testar certificado
openssl s_client -connect dubai.arxis.io:443 -servername dubai.arxis.io

# Verificar cipher suites
nmap --script ssl-enum-ciphers -p 443 dubai.arxis.io

# Verificar rating SSL
curl https://api.ssllabs.com/api/v3/analyze?host=dubai.arxis.io
```

### Health Check TLS
```bash
curl -k https://dubai.arxis.io/health
```

## Renovação Manual

Se precisar renovar manualmente:
```bash
sudo certbot renew --force-renewal
sudo systemctl reload nginx
```

## Backup de Certificados

```bash
# Backup
sudo tar -czf /opt/arxis/backups/certs-$(date +%Y%m%d).tar.gz /etc/letsencrypt/

# Restore
sudo tar -xzf /opt/arxis/backups/certs-YYYYMMDD.tar.gz -C /
```

## Monitoramento

Alertas configurados no Prometheus para:
- Expiração de certificados (< 30 dias)
- Falhas no TLS handshake
- Versões de protocolo antigas

## Referências

- Let's Encrypt: https://letsencrypt.org/
- SSL Labs: https://www.ssllabs.com/
- Mozilla SSL Config: https://ssl-config.mozilla.org/
