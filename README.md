# check_macos_updates

``` sh
$ check_macos_updates -h
A monitoring plugin that checks for available MacOS updates.

Thresholds are defined using monitoring plugin range syntax. Examples:
+------------------+-------------------------------------------------+
| Range definition | Generate an alert if x...                       |
+------------------+-------------------------------------------------+
| 10               | < 0 or > 10, (outside the range of {0 .. 10})   |
+------------------+-------------------------------------------------+
| 10:              | < 10, (outside {10 .. ∞})                       |
+------------------+-------------------------------------------------+
| ~:10             | > 10, (outside the range of {-∞ .. 10})         |
+------------------+-------------------------------------------------+
| 10:20            | < 10 or > 20, (outside the range of {10 .. 20}) |
+------------------+-------------------------------------------------+
| @10:20           | ≥ 10 and ≤ 20, (inside the range of {10 .. 20}) |
+------------------+-------------------------------------------------+


Usage: check_macos_updates [OPTIONS]

Options:
  -f, --force-manual         Force manual check with `softwareupdate -l` (slow)
  -w, --warning <WARNING>    Warning limit for number of updates available [default: 0]
  -c, --critical <CRITICAL>  Critical limit for number of updates available
  -h, --help                 Print help
  -V, --version              Print version

```

Use together with NRPE or similar, preferably with [Opsview](https://www.itrsgroup.com/products/infrastructure-monitoring).
