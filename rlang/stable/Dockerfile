FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm git rustup base-devel curl wget fd eza
RUN export CARGO_INSTALL_ROOT=/usr
RUN rustup default stable
RUN cargo install zuu
RUN export PATH=$PATH"
WORKDIR /app