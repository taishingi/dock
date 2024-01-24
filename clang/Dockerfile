FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm base base-devel distcc gettext brotli sdcc musl atool cppcheck tcc libcanberra gsound vala unzip zlib libzip gtk4 glib2 splint valgrind bc curl clang llvm ffmpeg cmake fd wget extra-cmake-modules ncurses git
RUN export LLVM_CONFIG_PATH=/usr/bin/llvm-config
RUN export PATH="$PATH"
WORKDIR /app