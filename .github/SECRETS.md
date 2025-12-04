# GitHub Actions Secrets Reference

Este repositório possui **27 secrets** configurados para uso em workflows CI/CD. Todos os valores foram sincronizados a partir do arquivo `.env` local usando o script `.github/scripts/sync-secrets.ps1`.

## 📋 Lista de Secrets

### DNS & Domain Services
- `PORKBUN_API_KEY` - Chave de API Porkbun para gerenciamento DNS
- `PORKBUN_SECRET_KEY` - Secret key Porkbun
- `CLOUDFLARE_API_KEY` - Token Cloudflare para CDN/DNS

### Database
- `MONGO_ATLAS_URI` - String de conexão MongoDB Atlas (inclui credenciais)

### Payment Services
- `PAYPAL_ID` - ID da conta PayPal
- `PAYPAL_TOKEN_API` - Token de autenticação PayPal
- `STRIPE_API` - Chave de API Stripe (test mode)

### AI & ML Services
- `OPENAI_API_KEY` - Chave OpenAI (projeto Dubai)
- `LANGSMITH_API_KEY` - Token LangSmith para tracing
- `HF_TOKEN` - Token Hugging Face Hub

### Developer Tools
- `ALT_GITHUB_USERNAME` - Username GitHub (prefixo ALT_ devido a restrição da plataforma)
- `ALT_GITHUB_TOKEN` - Personal Access Token GitHub (ALT_ prefix)
- `CARGO_REGISTRY_TOKEN` - Token crates.io para publicação Rust
- `SENTRY_TOKEN_API` - Token Sentry para error tracking
- `NGROK` - Token ngrok para túneis seguros

### Google Cloud Services
- `GCLOUD_API_TOKEN` - Token de API Google Cloud
- `GCLOUD_CLIENT` - OAuth Client ID Google
- `GCLOUD_SECRET_KEY` - OAuth Client Secret Google
- `GCLOUD_MAPS_ID` - Map ID para Google Maps Platform

### Other Services
- `GRAVATAR` - Token Gravatar
- `IMAZING` - Licença iMazing

### Gmail Accounts (App Passwords)
- `GMAIL_USER_AVILACARGASRAPIDAS` / `GMAIL_PASSWORD_APP_AVILACARGASRAPIDAS`
- `GMAIL_USER_NICOLASROSAAB` / `GMAIL_PASSWORD_APP_NICOLASROSAAB`
- `GMAIL_USER_FATURAMENTO` / `GMAIL_PASSWORD_APP_FATURAMENTO`

## 🔄 Sincronização

Para atualizar todos os secrets de uma vez:

```powershell
# Windows PowerShell
.\.github\scripts\sync-secrets.ps1
```

**Pré-requisitos:**
- GitHub CLI (`gh`) instalado e autenticado
- Permissões de escrita no repositório

## ⚠️ Notas Importantes

1. **Naming Restrictions**: GitHub Actions não permite secrets com prefixo `GITHUB_`. Por isso, usamos `ALT_GITHUB_*` para credenciais GitHub.

2. **Gmail App Passwords**: São senhas de aplicativo (não senhas principais). Cada conta Gmail possui seu próprio par user/password com sufixo identificador.

3. **Stripe Test Mode**: A chave `STRIPE_API` é do ambiente de testes (`rk_test_*`). Produção requer outra secret.

4. **Segurança**: Nunca commite o arquivo `.env` no repositório. Ele está listado no `.gitignore` por padrão.

## 🔗 Links Úteis

- [Ver secrets atuais](https://github.com/avilaops/Dubai/settings/secrets/actions)
- [Documentação GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Script de sincronização](.github/scripts/sync-secrets.ps1)

---

**Última sincronização**: 4 de dezembro de 2025
**Total de secrets**: 27
