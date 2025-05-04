FROM rust:latest AS builder

WORKDIR /usr/src/form-portal
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/form-portal/target/release/form_portal /usr/local/bin/form_portal
COPY wait-for-db.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/wait-for-db.sh

ENV RUST_LOG=info

EXPOSE 8080

CMD ["wait-for-db.sh", "form_portal", "--host", "0.0.0.0"]