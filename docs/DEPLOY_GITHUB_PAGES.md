# Guia de Publicação · Dubai Atlas (GitHub Pages)

Este front-end estático foi preparado para servir diretamente a partir de `docs/` no repositório. Siga os passos abaixo para publicar ou atualizar o site no GitHub Pages.

## 1. Pré-requisitos

- Branch principal com os arquivos novos (`docs/index.html`, `docs/assets`, `docs/data`).
- Arquivo `CNAME` já presente na raiz do repositório (`dubai.avilaops.com`).
- Permissões de administrador ou manutenção no repositório GitHub `dubai`.

## 2. Configurar GitHub Pages

1. Abra **Settings → Pages** no repositório.
2. Em **Source**, selecione:
   - **Branch:** `main` (ou a branch padrão).
   - **Folder:** `/docs`.
3. Salve. O GitHub irá provisionar o site em alguns minutos.
4. Confirme que o domínio personalizado está correto em **Custom domain** (`dubai.avilaops.com`). O arquivo `CNAME` na raiz garante consistência ao publicar via Pages.

> ℹ️ A primeira publicação pode levar até 5 minutos. Atualizações seguintes normalmente propagam em menos de 1 minuto.

## 3. Fluxo de Atualização

- Faça commits com quaisquer alterações adicionais na pasta `docs/` (conteúdo, estilo, dados JSON).
- Execute `cargo test` (opcional, mas recomendado) para garantir que nenhum fluxo back-end foi afetado.
- Envie a branch para o GitHub e faça merge na branch padrão.
- Aguarde o build automático do GitHub Pages concluir. Verifique o status em **Settings → Pages → GitHub Actions** caso exista workflow dedicado.

## 4. Observabilidade

- O front-end busca `docs/data/invoice-edp-2025-11.json`. Ajuste o arquivo ou adicione novos JSONs conforme a evolução do Atlas.
- Erros de carregamento são exibidos no topo da página como painel de falha. Use o console do navegador para detalhes (`F12`).
- O cache HTTP é desativado via `fetch(..., { cache: "no-cache" })`, garantindo leitura imediata de publicações.

## 5. Troubleshooting

| Sintoma | Causa provável | Ação sugerida |
| --- | --- | --- |
| Página 404/"Site not found" | Branch/folder incorretos ou build recente ainda não finalizado | Reconfirme fonte em Settings → Pages e aguarde 5 min |
| CNAME removido | GitHub Pages sobrescreve ao usar branch diferente | Confira se `CNAME` está na branch publicada e reconfigure domínio |
| JSON não carrega | Commit ausente ou publish em andamento | Verifique `docs/data/…` no branch, force reload (_Ctrl+Shift+R_) |
| Layout quebrado | Cache antigo de CSS | Limpe cache do navegador ou ajuste `app.js` para versionar assets |

## 6. Próximos Passos

- Adicionar novos `docs/data/*.json` para futuras faturas.
- Integrar GitHub Actions para validar JSON schema antes do deploy.
- Avaliar conversão do front-end em microsite com múltiplas faturas e comparativos.

Pronto! O front-end soberano do projeto Dubai está apto para publicação direta via GitHub Pages, sem dependências externas.
