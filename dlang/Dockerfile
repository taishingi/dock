FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm dub dtools dlang-dmd git curl wget fd eza
RUN export PATH="$PATH"
WORKDIR /app
CMD ["dub" , "build"]
CMD ["dub" , "test"]
CMD ["dub" , "lint"]
