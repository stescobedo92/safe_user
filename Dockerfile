FROM rust:latest AS builder
LABEL authors="stescobedo"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src ./src
COPY test ./test

RUN cargo build --release

# Stage 2: Runtime
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/safe_user ./

ENV DATABASE_URL=mssql://sa:Tester*31@mssql:1433/master
ENV JWT_SECRET=your_secret_key

EXPOSE 8080

CMD ["./safe_user"]



