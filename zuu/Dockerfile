FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm xterm git rustup base-devel curl wget fd eza cargo-audit cargo-auditable ncurses packer
RUN export TERM=xterm-256color 
RUN echo 'TERM=xterm-256color' >> /etc/environment
RUN rustup default stable
RUN export CARGO_INSTALL_ROOT="/usr"
RUN cargo install zuu@16.1.0
RUN export PATH="$PATH"
WORKDIR /app
CMD [ "zuu" ]
