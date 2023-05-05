use check_macos_updates::*;
use clap::Parser;
use plist::from_file;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
    /// Force manual check with `softwareupdate -l` (slow)
    #[arg(short, long)]
    force_manual: bool,

    /// Return critical if updates are available
    #[arg(short, long)]
    critical_on_updates: bool,
}

fn exit_with_message(status: Status) {
    println!("{}", status);
    process::exit(status.to_int());
}

/// Check for macOS updates. Returns warning if updates are available.
fn main() {
    let mut status = Status::Unknown(UnkownVariant::NotMacOS);

    if !cfg!(target_os = "macos") {
        exit_with_message(status)
    }

    let args = Args::parse();

    if args.force_manual {
        status = check_output(&get_output());
    } else {
        let software_update_plist = from_file(PLIST_FILE);

        status = match software_update_plist {
            Ok(u) => determine_updates(&u),
            Err(_) => Status::Unknown(UnkownVariant::UnableToDetermineUpdates),
        };
    }

    if args.critical_on_updates {
        if let Status::Warning = status {
            status = Status::Critical;
        }
    }

    exit_with_message(status);
}
