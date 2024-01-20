FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm base base-devel distcc gettext brotli atool cppcheck tcc libcanberra gsound vala unzip zlib libzip musl gtk4 glib2 splint valgrind bc curl clang llvm cmake fd wget cmake extra-cmake-modules ncurses git
RUN export LLVM_CONFIG_PATH=/usr/bin/llvm-config
WORKDIR /app