FROM messense/rust-musl-cross:x86_64-musl AS builder

#Set the following value so we can build our codebase without live database connection
ENV SQLX_OFFLINE=true
WORKDIR /src

# Copy the entire project including ruva
COPY . .

# Ensure sqlx-data.json exists before build
RUN cargo build --release --target x86_64-unknown-linux-musl

# stage for backend binary
FROM alpine:latest AS command_server

# Install postgresql-client for database migrations
RUN apk add --no-cache postgresql-client

# Copy migrations
COPY --from=builder /src/migrations /migrations

# Copy binary file
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/command_server /command_server

ENV SQLX_OFFLINE=true
CMD [ "/command_server" ]
EXPOSE 80
LABEL service="command_server"
