FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm xterm git rustup base-devel curl wget fd eza cargo-audit ncurses
RUN export TERM=xterm-256color
RUN echo 'TERM=xterm-256color' >> /etc/environment
RUN rustup default stable
ENV CARGO_INSTALL_ROOT=/usr
RUN cargo install zuu@7.2.1 rsbadges
ENV PATH="$HOME/.cargo/bin:$PATH"
WORKDIR /app
CMD [ "zuu" ]
