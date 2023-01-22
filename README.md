# nagios_check_site_content
    Nagios plugin written in Rust, used to check website for specified content.

    sends GET to specified url and checks the response body for a specified keyword (Nagios compatible)

    Format;
    ./check_site **%username% **%password% %url% %keyword%

    note:
    installable as Nagios plugin
    use environment variables to pass credentials

### One-line Nagios Plugin Install
builds in /tmp and copies bin to /usr/local/nagios/libexec/
```agsl
curl -H 'Cache-Control: no-cache, no-store' https://raw.githubusercontent.com/helloimalemur/nagios_check_site_content/master/src/install_as_nagios_plugin.sh | sh
```

### Normal build
    git clone https://github.com/helloimalemur/nagios_check_site_content.git
    cd check_site_for_keyword/
    cargo build
    export CSUSER="<<username>>"
    export CSCRED="<<password>>"
    target/debug/check_site_for_keyword "$CSUSER" "$CSCRED" https://elastic:9200 build_hash
