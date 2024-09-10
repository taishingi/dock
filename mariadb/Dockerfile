FROM archlinux/archlinux:latest
RUN pacman -Syyu --noconfirm mariadb  mariadb-clients rustup curl base-devel cmake
RUN rustup default stable
WORKDIR /app
COPY src .
EXPOSE 3050:3050
RUN export CARGO_TARGET_DIR=/usr/bin
RUN cargo build --release --features mariadb
CMD ["/usr/bin/chronos","--features","mariadb"]
