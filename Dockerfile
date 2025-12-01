# Dockerfile - Dubai Project
# Deploy no Servidor Arxis

FROM rust:1.75-slim as builder

# Instalar dependências de sistema (se necessário)
RUN apt-get update && apt-get install -y \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /app

# Copiar manifesto
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Copiar código fonte
COPY src ./src
COPY tests ./tests

# Build em modo release
RUN cargo build --release --bin dubai-project

# Imagem final (Alpine para tamanho mínimo)
FROM debian:bookworm-slim

# Instalar CA certificates para HTTPS
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -m -u 1000 arxis

# Criar diretórios necessários
RUN mkdir -p /var/lib/arxis/dubai/data \
             /var/lib/arxis/dubai/backups \
             /var/log/arxis/dubai \
             /etc/arxis/certs && \
    chown -R arxis:arxis /var/lib/arxis /var/log/arxis /etc/arxis

# Copiar binário
COPY --from=builder /app/target/release/dubai-project /usr/local/bin/

# Usar usuário não-root
USER arxis

# Variáveis de ambiente padrão
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expor portas
EXPOSE 8080 9090

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Comando de inicialização
CMD ["dubai-project"]
