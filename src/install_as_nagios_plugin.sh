#!/bin/bash

if [[ -f /usr/local/nagios/libexec/check_disk ]]; then
  #  rm -rf /tmp/notify_via_discord/ 2>/dev/null;
    cd /tmp/;
    git clone https://github.com/helloimalemur/nagios_check_site_content.git;
    cd nagios_check_site_content/;
    cargo build;
    sudo cp target/debug/check_site /usr/local/nagios/libexec/;
else
  echo "dir '/usr/local/nagios/libexec/' does not exist";
fi;
