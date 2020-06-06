# The builder container
FROM rust:slim as builder
WORKDIR /app

# Copy source code over
COPY . .
# Compile the app
RUN cargo build --release

# The app container
FROM gcr.io/distroless/cc-debian10
LABEL Author="Zachary Kohnen"
WORKDIR /app

# Copy binary to the app
COPY --from=builder /app/target/release/whs_helper_bot /app

# Copy app files
COPY --from=builder /app/.env /app/Config.toml /app/

# Run the app
CMD ["./whs_helper_bot"]