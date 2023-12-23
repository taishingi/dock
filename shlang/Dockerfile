FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm git base-devel curl wget coreutils util-linux diffutils fd eza
WORKDIR /app
RUN wget https://github.com/bach-sh/bach/raw/master/bach.sh