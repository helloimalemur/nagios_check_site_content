#!/bin/bash

if [[ -n /usr/local/nagios/libexec/ ]]; then
  echo "dir '/usr/local/nagios/libexec/' does not exist";
else
#  rm -rf /tmp/notify_via_discord/ 2>/dev/null;
  cd /tmp/;
  git clone https://github.com/helloimalemur/rustyscripts.git;
  cd rustyscripts/check_site_for_keyword/;
  cargo build;
  sudo cp target/debug/check_site_for_keyword /usr/local/nagios/libexec/;
fi;
