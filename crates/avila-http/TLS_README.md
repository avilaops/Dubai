# AvilaHttp TLS Implementation

## 🔒 Implementação TLS Nativa

O AvilaHttp agora suporta **HTTPS/TLS nativo** sem dependências externas!

### ✨ Características

- **TLS 1.2 Handshake** completo
- **SNI (Server Name Indication)** para virtual hosts
- Suporte a múltiplos **Cipher Suites**:
  - `TLS_RSA_WITH_AES_128_GCM_SHA256`
  - `TLS_RSA_WITH_AES_128_CBC_SHA`
  - `TLS_RSA_WITH_AES_256_CBC_SHA`
- **Zero dependências externas** - 100% Rust puro

### 🚀 Uso

```rust
use avila_http::HttpClient;

let client = HttpClient::new();

// HTTPS automático quando URL começa com https://
let response = client.get("https://www.google.com")?;

println!("Status: {}", response.status_code);
println!("Body: {}", response.body);
```

### 📋 Fluxo do Handshake TLS

1. **ClientHello** → Envia versão TLS, cipher suites, random, SNI
2. **ServerHello** ← Recebe cipher suite escolhido, random do servidor
3. **Certificate** ← Recebe certificado X.509 do servidor
4. **ServerHelloDone** ← Confirma fim das mensagens do servidor
5. **ClientKeyExchange** → Envia pre-master secret (encriptado com RSA do certificado)
6. **ChangeCipherSpec** → Notifica mudança para comunicação encriptada
7. **Finished** → Envia hash verificável de todas as mensagens
8. **ChangeCipherSpec** ← Servidor confirma mudança
9. **Finished** ← Servidor confirma handshake

### 🔐 Segurança

**⚠️ NOTA IMPORTANTE**: Esta é uma implementação educacional/demonstrativa.

Para **produção real**, é altamente recomendado usar:
- `rustls` - TLS puro em Rust
- `native-tls` - Wrapper do TLS do sistema operacional
- `openssl` - Binding do OpenSSL

**Limitações atuais**:
- Gerador de random simplificado (não CSPRNG)
- Validação de certificado não implementada
- Encriptação de dados de aplicação simplificada
- Não implementa todos os cipher suites modernos
- Não suporta TLS 1.3 ainda

### 🛠️ Estrutura Técnica

#### TLS Record Layer

```
+----------+----------+----------+----------+
| Type (1) | Ver (2)  | Len (2)  | Data (n) |
+----------+----------+----------+----------+
```

- **Type**: ContentType (Handshake=22, ApplicationData=23, etc)
- **Version**: Major.Minor (TLS 1.2 = 3.3)
- **Length**: Tamanho do payload
- **Data**: Payload (handshake messages ou dados encriptados)

#### SNI Extension

```
Extension Type: 0x0000 (server_name)
Server Name Type: 0x00 (host_name)
Server Name: www.example.com
```

Essencial para servidores com múltiplos domínios no mesmo IP.

### 📊 Testes

```bash
# Testes unitários
cargo test -p avila-http

# Teste HTTPS real (ignorado por padrão)
cargo test -p avila-http test_real_https_request -- --ignored
```

### 🔄 Comparação: HTTP vs HTTPS

| Característica | HTTP | HTTPS |
|---------------|------|-------|
| Porta padrão | 80 | 443 |
| Encriptação | ❌ Não | ✅ Sim (TLS) |
| Handshake | Simples | TLS multi-step |
| Validação | Nenhuma | Certificado X.509 |
| SNI | N/A | ✅ Suportado |

### 📝 Exemplo Completo

```rust
use avila_http::HttpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new()
        .with_timeout(60); // 60 segundos

    // HTTP simples
    let http_response = client.get("http://example.com")?;
    println!("HTTP Status: {}", http_response.status_code);

    // HTTPS com TLS
    let https_response = client.get("https://www.rust-lang.org")?;
    println!("HTTPS Status: {}", https_response.status_code);
    println!("Body length: {}", https_response.body.len());

    Ok(())
}
```

### 🎯 Roadmap Futuro

- [ ] TLS 1.3 support
- [ ] Validação completa de certificado X.509
- [ ] Certificate pinning
- [ ] OCSP stapling
- [ ] Session resumption
- [ ] Cipher suite moderno (ChaCha20-Poly1305)
- [ ] Suporte a client certificates
- [ ] CSPRNG adequado para produção

### 📚 Referências

- [RFC 5246 - TLS 1.2](https://tools.ietf.org/html/rfc5246)
- [RFC 6066 - TLS Extensions (SNI)](https://tools.ietf.org/html/rfc6066)
- [RFC 8446 - TLS 1.3](https://tools.ietf.org/html/rfc8446)

---

**Powered by Nícolas Ávila** 🦀
