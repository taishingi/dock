FROM ubuntu:24.04
RUN apt-get update && apt-get dist-upgrade -y && apt-get install -y  php8.2-apcu php-apcu php-apcu-all-dev php-symfony-polyfill-apcu composer php-curl php-gd php-intl php-json php-mcrypt php-mbstring php-xml php-zip php-imagick php-sqlite3 php-mysql php-pgsql php-imap php-common php-cli libapache2-mod-php postgresql wget fd-find git-all php-apcu curl sqlite3 mariadb-server php-gmp php-xdebug vim ranger
RUN ln -sv /usr/bin/fdfind /usr/local/bin/fd 
RUN phpenmod apcu xdebug 
ADD xdebug.ini /etc/php/8.2/mods-available/xdebug.ini	
RUN export PATH="$PATH"
WORKDIR /app
CMD ["composer", "install"]
CMD ["php", "-d", "memory_limit=-1", "vendor/bin/phpunit"]
CMD ["php", "-d", "memory_limit=-1", "vendor/bin/phpstan","analyse"]
