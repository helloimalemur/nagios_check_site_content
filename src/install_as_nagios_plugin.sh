#!/bin/bash

if [[ -f /usr/local/nagios/libexec/check_disk ]]; then
    rm -rf /tmp/nagios_check_site_content/ 2>/dev/null;
    cd /tmp/;
    git clone https://github.com/helloimalemur/nagios_check_site_content.git;
    cd nagios_check_site_content/;
    cargo build;
    cp target/debug/check_site /usr/local/nagios/libexec/;
    cp src/examples/check_elastic.sh /usr/local/nagios/libexec/;
    cp src/examples/check_kibana.sh /usr/local/nagios/libexec/;
    chmod +x /usr/local/nagios/libexec/*.sh;
else
  echo "dir '/usr/local/nagios/libexec/' does not exist";
fi;
