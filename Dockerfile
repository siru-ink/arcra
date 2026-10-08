## Container 1: Compile the rust code using musl to prevent external dependencies
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev git

# Copy sources into container 1 for building
RUN mkdir /arcra
RUN mkdir /graka
RUN mkdir /data
COPY Cargo.toml Cargo.lock ./arcra/
COPY src ./arcra/src
COPY migrations ./arcra/migrations
COPY templates ./arcra/templates
COPY .sqlx ./arcra/.sqlx

WORKDIR /arcra
RUN cargo build --release

WORKDIR /graka
RUN git clone https://code.siru.ink/siru-ink/graka.git .
RUN cargo build --release

# Container 2: Minimal output
FROM scratch

# Copy needed runtime sources & empty directories
COPY --from=builder /arcra/target/release/arcra /arcra
COPY --from=builder /arcra/migrations /migrations
COPY --from=builder /arcra/templates /templates
COPY --from=builder /data /data
COPY --from=builder /graka/target/release/graka /graka

# Metainfo Setup
VOLUME ["/data"]
ENTRYPOINT ["/arcra"]
EXPOSE 80
ENV HEALTHCHECK_PORT=80
ENV HEALTHCHECK_PATH=healthcheck
HEALTHCHECK --start-period=10s CMD ["/graka"]
