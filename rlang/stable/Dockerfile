FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm git rustup base-devel curl wget fd eza
RUN rustup default stable
RUN export PATH=$PATH
WORKDIR /app