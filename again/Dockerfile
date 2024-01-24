FROM archlinux/archlinux
LABEL authors="Willy Micieli"
RUN pacman -Syyu --noconfirm git rustup ranger fd wget packer base base-devel
RUN export RUSTUP_TOOLCHAIN=stable
RUN export CARGO_TARGET_DIR=target
RUN rustup default stable
RUN git clone https://github.com/taishingi/continuous-testing.git testing && cd testing && cargo build --release && install -Dm0755 -t "/usr/bin" "target/release/again" && cd .. && rm -rf testing
RUN export PATH="$PATH"
WORKDIR /app
CMD ["again"]