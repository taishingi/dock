FROM ubuntu:23.04
RUN apt-get update && apt-get install -y git-all php-fpm php-php-gettext sqlite3 postgresql-client php-sockets openssl php-gd php-xmlreader openssl php-intl fd-find php-ftp calendar pkg-config build-essential autoconf bison re2c libxml2-dev libsqlite3-dev php-curl php-gd php-intl php-cli wget php-xml php-json php-mbstring php-xml php-zip composer apache2 php libapache2-mod-php mariadb-server php-mysql php-xdebug
EXPOSE 80:80
EXPOSE 433:433
EXPOSE 3000:3000
EXPOSE 8000:8000
EXPOSE 8080:8080
WORKDIR /app