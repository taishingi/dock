FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm git rustup base-devel curl wget fd eza
RUN rustup default nightly
RUN export PATH=$PATH
WORKDIR /app