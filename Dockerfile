FROM rust:1.71
WORKDIR /src/
COPY . .
RUN cargo install --path .
RUN cargo build
CMD [ "./target/release/mandelbrot-explorer-rs" ]
