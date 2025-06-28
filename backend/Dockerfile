# ---------- Builder Stage ----------
FROM rust:alpine AS builder

# Install musl-dev and static OpenSSL libraries
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create app dir and copy code
WORKDIR /app
COPY . .

# Build statically linked binary with OpenSSL static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---------- Runtime Stage ----------
FROM scratch

# Copy the correct binary name (defined in [[bin]] section)
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /backend

# Copy CA certificates for HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENTRYPOINT ["/backend"]