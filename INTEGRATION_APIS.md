# Sistema de Integração com APIs de Dubai

## 🎯 Visão Geral

Sistema completo para coletar dados de imóveis de Dubai de múltiplas fontes (APIs governamentais e portais imobiliários) e armazená-los no MongoDB Atlas.

## 🏗️ Arquitetura

```
┌─────────────────────────────────────────────────────────────┐
│                    FONTES DE DADOS                          │
├─────────────────────────────────────────────────────────────┤
│  Bayut API          │  Property Finder  │  Dubai Land Dept  │
│  Dubizzle API       │  UAE Pass        │  Dubai Stats      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              fetch-dubai-data (Rust Binary)                 │
│  • Coleta dados de todas as APIs                           │
│  • Parseia e normaliza estrutura                           │
│  • Calcula distâncias e métricas                           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  MongoDB Atlas                              │
│  Database: dubai                                            │
│  Collection: properties                                     │
│  • properties (imóveis)                                     │
│  • free_zones (zonas francas)                               │
│  • landmarks (pontos de referência)                         │
│  • market_statistics (estatísticas)                         │
│  • transactions (transações DLD)                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              GitHub Pages (dubai.avilaops.com)              │
│  • Consome dados do MongoDB Atlas                          │
│  • Exibe propriedades interativas                          │
│  • Atualização automática via GitHub Actions               │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Configuração

### 1. MongoDB Atlas

Já configurado no `.env`:

```env
MONGO_ATLAS_URI=mongodb+srv://nicolasrosaab_db_user:Gio4EAQhbEdQMISl@cluster0.npuhras.mongodb.net/
MONGODB_ATLAS_APP_ID=application-0-djjmn
MONGODB_ATLAS_API_KEY=Gio4EAQhbEdQMISl
MONGODB_ATLAS_CLUSTER=cluster0
MONGODB_ATLAS_DATABASE=dubai
MONGODB_ATLAS_COLLECTION=properties
```

### 2. APIs de Propriedades (Opcional - a configurar)

#### Bayut API
```env
BAYUT_API_KEY=
BAYUT_API_SECRET=
```

**Como obter:**
1. Acesse https://www.bayut.com
2. Entre em contato com equipe comercial: business@bayut.com
3. Solicite acesso ao Developer Portal
4. Gere suas credenciais de API

#### Property Finder API
```env
PROPERTYFINDER_CLIENT_ID=
PROPERTYFINDER_CLIENT_SECRET=
```

**Como obter:**
1. Acesse https://www.propertyfinder.ae/en/developers
2. Registre sua aplicação
3. Configure OAuth 2.0
4. Copie Client ID e Secret

#### Dubizzle API
```env
DUBIZZLE_API_KEY=
```

**Como obter:**
1. Acesse https://dubai.dubizzle.com
2. Solicite API key para desenvolvedores

### 3. APIs Governamentais (Opcional)

#### Dubai Land Department
```env
DLD_API_KEY=
DLD_API_URL=https://dubailand.gov.ae/api
```

**Como obter:**
1. Acesse https://dubailand.gov.ae
2. Registre-se no portal de desenvolvedores
3. Solicite aprovação para API

**Dados públicos disponíveis:**
- https://dubailand.gov.ae/en/open-data/real-estate-transactions
- Exportação CSV/JSON sem necessidade de API key

#### UAE Pass (Autenticação Governo)
```env
UAE_PASS_CLIENT_ID=
UAE_PASS_CLIENT_SECRET=
```

## 📦 Binários Rust

### 1. `sync-to-atlas`

Envia dados locais (JSON) para MongoDB Atlas.

**Uso:**
```powershell
.\sync-atlas.ps1 -Build
```

**O que faz:**
- Carrega `docs/data/dubai-properties.json`
- Converte para documentos MongoDB
- Insere no Atlas:
  - 12 propriedades
  - 3 free zones
  - 3 landmarks
  - Estatísticas de mercado

### 2. `fetch-dubai-data`

Coleta dados de APIs externas e salva no MongoDB Atlas.

**Uso:**
```powershell
.\fetch-apis.ps1 -Build
```

**O que faz:**
- Verifica APIs configuradas
- Faz requests HTTP para cada fonte
- Parseia JSON responses
- Calcula coordenadas e distâncias
- Salva documentos no MongoDB Atlas

## 🚀 Fluxo de Trabalho

### Primeira sincronização (Dados locais):

```powershell
# 1. Compilar e executar sync
.\sync-atlas.ps1 -Build

# 2. Verificar no MongoDB Atlas
# https://cloud.mongodb.com
```

### Fetch de APIs (Quando configuradas):

```powershell
# 1. Configurar APIs no .env
# Copiar credenciais de .env.apis

# 2. Executar fetch
.\fetch-apis.ps1 -Build

# 3. Dados atualizados automaticamente no Atlas
```

### Atualização do Site:

```powershell
# O site em dubai.avilaops.com pode ser atualizado para
# consumir dados diretamente do MongoDB Atlas

# Ou exportar do Atlas para JSON e commitar:
# 1. Export do Atlas
# 2. Salvar em docs/data/dubai-properties.json
# 3. Commit e push
# 4. GitHub Actions deploya automaticamente
```

## 📊 Estrutura de Dados no MongoDB

### Documento de Propriedade
```json
{
  "doc_type": "property",
  "source": "bayut",
  "id": "dp-001",
  "title": "Luxury 2BR Apartment in Dubai Marina",
  "price": 1850000,
  "price_per_sqm": 15416.67,
  "currency": "AED",
  "location": "Dubai Marina",
  "coordinates": {
    "lat": 25.0805,
    "lon": 55.1399
  },
  "bedrooms": 2,
  "bathrooms": 2,
  "area_sqm": 120,
  "property_type": "Apartment",
  "url": "https://www.bayut.com/...",
  "distance_to_burj_khalifa_km": 14.2,
  "building": "Marina Pinnacle Tower",
  "year_built": 2015,
  "ready_to_move": "yes",
  "features": "Sea view, Balcony, Gym access, Swimming pool",
  "synced_at": 1733356800.0
}
```

### Documento de Free Zone
```json
{
  "doc_type": "free_zone",
  "name": "Dubai Multi Commodities Centre (DMCC)",
  "location": "Jumeirah Lakes Towers",
  "website": "https://www.dmcc.ae",
  "cost_min_aed": 15000,
  "cost_max_aed": 50000,
  "benefits": "100% foreign ownership; 0% tax; Capital repatriation",
  "business_types": "Trading, Services, Consulting"
}
```

### Documento de Landmark
```json
{
  "doc_type": "landmark",
  "key": "burj_khalifa",
  "name": "Burj Khalifa",
  "coordinates": {
    "lat": 25.1972,
    "lon": 55.2744
  }
}
```

### Documento de Estatísticas
```json
{
  "doc_type": "market_statistics",
  "average_price_aed": 2909166.67,
  "median_price_aed": 2450000,
  "min_price_aed": 420000,
  "max_price_aed": 8500000,
  "average_price_per_sqm_aed": 15895.49,
  "areas_covered": "Dubai Marina, Downtown Dubai, Palm Jumeirah...",
  "updated_at": 1733356800.0
}
```

## 🔄 Automação

### Opção 1: GitHub Actions (Recomendado)

Criar `.github/workflows/fetch-dubai-data.yml`:

```yaml
name: Fetch Dubai Data

on:
  schedule:
    - cron: '0 0 * * *'  # Diariamente às 00:00 UTC
  workflow_dispatch:

jobs:
  fetch:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build
        run: cargo build --release --bin fetch-dubai-data
      
      - name: Fetch Data
        env:
          MONGO_ATLAS_URI: ${{ secrets.MONGO_ATLAS_URI }}
          MONGODB_ATLAS_APP_ID: ${{ secrets.MONGODB_ATLAS_APP_ID }}
          MONGODB_ATLAS_API_KEY: ${{ secrets.MONGODB_ATLAS_API_KEY }}
          BAYUT_API_KEY: ${{ secrets.BAYUT_API_KEY }}
          DLD_API_KEY: ${{ secrets.DLD_API_KEY }}
        run: ./target/release/fetch-dubai-data
```

### Opção 2: Cron Job (Servidor)

```bash
# Executar diariamente às 2h da manhã
0 2 * * * cd /path/to/dubai && ./fetch-apis.ps1
```

## 📖 Referências

### APIs Documentação
- **Bayut**: Contato comercial necessário
- **Property Finder**: https://www.propertyfinder.ae/en/developers
- **Dubai Land Department**: https://dubailand.gov.ae/en/open-data
- **Dubai Statistics Center**: https://www.dsc.gov.ae
- **UAE Pass**: https://uaepass.ae

### Ferramentas Avila
- `avila-http`: Cliente HTTP nativo
- `avila-json`: Parser JSON sem dependências
- `avila-mongo`: Cliente MongoDB Atlas
- `avila-geo`: Cálculos geográficos (Haversine)
- `avila-cache`: Cache em memória

## 🎯 Status Atual

✅ **Implementado:**
- MongoDB Atlas conectado
- Dados locais (12 propriedades, 3 free zones, landmarks)
- Binário `sync-to-atlas` funcional
- Binário `fetch-dubai-data` estruturado
- Scripts PowerShell para automação
- Site em dubai.avilaops.com exibindo dados

⏳ **A Configurar:**
- API keys das plataformas imobiliárias
- OAuth para Property Finder
- API key do Dubai Land Department
- Automação via GitHub Actions

## 💡 Próximos Passos

1. **Imediato:**
   ```powershell
   # Sincronizar dados locais com Atlas
   .\sync-atlas.ps1 -Build
   ```

2. **Curto Prazo:**
   - Solicitar API keys dos portais
   - Configurar no `.env`
   - Executar `.\fetch-apis.ps1 -Build`

3. **Médio Prazo:**
   - Configurar GitHub Actions para fetch automático
   - Implementar webhook para atualizações em tempo real
   - Adicionar mais fontes de dados (Dubizzle, etc)

4. **Longo Prazo:**
   - Dashboard de monitoramento de preços
   - Alertas para novas propriedades
   - Machine learning para previsão de preços
