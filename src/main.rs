use check_macos_updates::*;
use clap::Parser;
use nagios_range::NagiosRange as ThresholdRange;
use plist::from_file;
use std::process;

const ABOUT_TEXT: &str = r#"
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
+------------------+-------------------------------------------------+"#;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None, about = ABOUT_TEXT)]
struct Args {
    /// Force manual check with `softwareupdate -l` (slow)
    #[arg(short, long)]
    force_manual: bool,

    /// Warning limit for number of updates available
    #[arg(short, long, default_value = "0")]
    warning: Option<String>,

    /// Critical limit for number of updates available
    #[arg(short, long)]
    critical: Option<String>,
}

fn exit_with_message(status: Status) -> ! {
    println!("{}", status);
    process::exit(status.to_int());
}

/// Check for macOS updates. Returns warning if updates are available.
fn main() {
    // According to monitoring-plugins guidelines, exit code 3 is used for "UNKNOWN" and
    // should be used for the --help and --version flags.
    let args = Args::try_parse().unwrap_or_else(|e| match e.kind() {
        clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
            print!("{}", e);
            std::process::exit(3);
        }
        _ => exit_with_message(Status::Unknown(UnkownVariant::ClapError(e.to_string()))),
    });

    if !cfg!(target_os = "macos") {
        exit_with_message(Status::Unknown(UnkownVariant::NotMacOS))
    }

    if args.warning.is_none() && args.critical.is_none() {
        exit_with_message(Status::Unknown(UnkownVariant::NoThresholds))
    }

    let mut warning: Option<ThresholdRange> = None;

    if let Some(w) = args.warning {
        let w_range = ThresholdRange::from(&w);
        match w_range {
            Ok(r) => warning = Some(r),
            Err(e) => exit_with_message(Status::Unknown(UnkownVariant::RangeParseError(w, e))),
        }
    }

    let mut critical: Option<ThresholdRange> = None;

    if let Some(c) = args.critical {
        let c_range = ThresholdRange::from(&c);
        match c_range {
            Ok(r) => critical = Some(r),
            Err(e) => exit_with_message(Status::Unknown(UnkownVariant::RangeParseError(c, e))),
        }
    }

    let thresholds = Thresholds { warning, critical };

    if args.force_manual {
        let check_status = check_softwareupdate_output(&softwareupdate_output(), &thresholds);
        exit_with_message(check_status)
    } else {
        let software_update_plist = from_file(PLIST_FILE);
        let check_status = match software_update_plist {
            Ok(u) => determine_updates(&u, &thresholds),
            Err(_) => Status::Unknown(UnkownVariant::UnableToParsePlist),
        };
        exit_with_message(check_status)
    }
}
