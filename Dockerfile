FROM rust:1.71 as builder
RUN apt update && apt install -y ffmpeg
WORKDIR /work/
COPY . .
RUN cargo build --release && ./target/release/mandelbrot-explorer-rs
CMD [ "ffmpeg", "-r", "30", "-i", "./dist/%08d.png", "-vcodec", "libx264", "-pix_fmt", "yuv420p", "-r", "60", "./fruit.mp4" ]
