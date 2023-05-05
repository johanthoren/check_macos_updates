# check_macos_updates
Check if MacOS system updates are available.

``` sh
Usage: check_macos_updates [OPTIONS]

Options:
  -f, --force-manual         Force manual check with `softwareupdate -l` (slow)
  -c, --critical-on-updates  Return critical if updates are available
  -h, --help                 Print help
  -V, --version              Print version
```

Use together with nrpe or similar.
