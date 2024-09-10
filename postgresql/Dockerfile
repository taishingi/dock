FROM archlinux/archlinux:latest
RUN pacman -Syyu --noconfirm postgresql rustup curl base-devel cmake
RUN rustup default stable
WORKDIR /app
COPY src .
EXPOSE 5050:5050
RUN export CARGO_TARGET_DIR=/usr/bin
RUN cargo build --release --features postgres
CMD ["/usr/bin/chronos","--features","postgres"]
