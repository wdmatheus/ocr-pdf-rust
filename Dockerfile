ARG RUST_VERSION=1.83.0

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ENV RUSTFLAGS="-C target-feature=+neon"
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=console_app,target=console_app \
    --mount=type=bind,source=ocr_pdf,target=ocr_pdf \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release -p api && \
    mkdir -p /app/dist && \
    cp ./target/release/api /app/dist/

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.

FROM debian:bookworm-slim AS final

WORKDIR /app

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser && \
    chown -R appuser:appuser /app && \
    apt-get update --no-install-recommends && \
    apt-get install -y ocrmypdf tesseract-ocr-por --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /app/dist/api /app/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/app/api"]
