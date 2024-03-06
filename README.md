# check_macos_updates

A Nagios compatible plugin to check if macOS system updates are available.

``` sh
Usage: check_macos_updates [OPTIONS]

Options:
  -f, --force-manual         Force manual check with `softwareupdate -l` (slow)
  -h, --help                 Print help
  -V, --version              Print version
```

Use together with nrpe or similar, preferably with [Opsview](https://www.itrsgroup.com/products/infrastructure-monitoring).
