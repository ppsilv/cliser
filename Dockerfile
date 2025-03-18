# Estágio de construção
FROM rust:latest as builder

# Cria o diretório de trabalho
WORKDIR /usr/src/app

# Copia os arquivos do projeto
COPY . .

# Compila o binário específico (server)
RUN cargo build --release --bin server

# Estágio final (imagem menor)
# Use a mesma versão do Ubuntu que o seu sistema host
FROM ubuntu:24.04

# Copia o binário compilado
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/server
COPY config.json /usr/local/bin/config.json
COPY configcli.json /usr/local/bin/configcli.json

# Cria o link simbólico para o syslog
RUN mkdir -p /run/systemd/journal && ln -s /run/systemd/journal/dev-log /dev/log
	
# Define o comando padrão
#CMD ["server"]
CMD ["/usr/local/bin/server"]

