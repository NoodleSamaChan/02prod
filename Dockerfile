#Builder stage
FROM rust:1.78.0 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

#Runtime stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update -y \
    &&apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zer02prod zer02prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT [ "./zer02prod" ]