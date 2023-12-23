FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm git rustup base-devel curl wget fd eza
RUN rustup default beta
RUN export PATH=$PATH:$HOME/.cargo/bin