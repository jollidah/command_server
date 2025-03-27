FROM messense/rust-musl-cross:x86_64-musl AS builder

#Set the following value so we can build our codebase without live database connection
ENV SQLX_OFFLINE=true 
WORKDIR /src
COPY . .

# Before running this, make sure sqlx-data.json exists!
RUN cargo build --release --target x86_64-unknown-linux-musl

# stage for backend binary 
FROM alpine:latest AS BE
COPY --from=builder /src/migrations /migrations

# COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/BE /BE
ENV SQLX_OFFLINE=true 
CMD [ "/BE" ]
EXPOSE 80
LABEL service="BE"
