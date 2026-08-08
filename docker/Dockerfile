FROM rust:1.94-trixie AS builder
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

COPY Cargo.toml ./
COPY Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch --locked

COPY index.html ./
COPY src ./src
COPY static ./static
COPY style ./style
RUN trunk build --release

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]

# docker build -f docker/Dockerfile -t yew-starter:latest .
# docker run --rm -p 8080:80 yew-starter:latest
