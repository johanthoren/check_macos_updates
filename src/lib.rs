use serde::Deserialize;
use std::fmt;
use std::process::{self, Output};

pub const PLIST_FILE: &str = "/Library/Preferences/com.apple.SoftwareUpdate.plist";

#[derive(Debug, PartialEq)]
pub enum UnkownVariant {
    NotMacOS,
    UnableToDetermineUpdates,
    UnableToParsePlist,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Warning(usize),
    Critical(usize),
    Unknown(UnkownVariant),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Ok => write!(f, "OK - No updates available"),
            Status::Warning(n) => write!(f, "WARNING - Updates available: {}", n),
            Status::Critical(n) => write!(f, "CRITICAL - Updates available: {}", n),
            Status::Unknown(UnkownVariant::NotMacOS) => {
                write!(f, "UNKNOWN - Not running on macOS")
            }
            Status::Unknown(UnkownVariant::UnableToDetermineUpdates) => {
                write!(f, "UNKNOWN - Unable to determine available updates")
            }
            Status::Unknown(UnkownVariant::UnableToParsePlist) => {
                write!(f, "UNKNOWN - Unable to parse plist file")
            }
        }
    }
}

impl Status {
    pub fn to_int(&self) -> i32 {
        match self {
            Status::Ok => 0,
            Status::Warning(_) => 1,
            Status::Critical(_) => 2,
            Status::Unknown(_) => 3,
        }
    }
}

// See tests/plist_examples.rs for examples of the plist file.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoftwareUpdate {
    #[serde(default)]
    pub automatic_check_enabled: bool,
    // #[serde(default)]
    // pub automatic_download: bool,
    // pub last_successful_date: String,
    // pub last_attempt_system_version: String,
    pub last_updates_available: u8,
    // pub last_recommended_updates_available: u8,
    // pub last_attempt_build_version: String,
    // pub recommended_updates: Vec<String>,
    // pub last_full_successful_date: String,
    // pub primary_languages: Vec<String>,
    // pub last_session_successful: bool,
    // pub last_background_successful_date: String,
    // pub last_result_code: u8,
}

pub fn softwareupdate_output() -> Result<Output, std::io::Error> {
    process::Command::new("softwareupdate").arg("-l").output()
}

pub fn check_softwareupdate_output(output: &Result<Output, std::io::Error>) -> Status {
    match output {
        Ok(output) => {
            let output_stderr = String::from_utf8_lossy(&output.stderr);
            let output_stdout = String::from_utf8_lossy(&output.stdout);

            if output_stderr.contains("No new software available.") {
                Status::Ok
            } else {
                let n = output_stdout
                    .lines()
                    .filter(|l| l.contains("* Label:"))
                    .count();
                Status::Warning(n)
            }
        }
        Err(_) => Status::Unknown(UnkownVariant::UnableToDetermineUpdates),
    }
}

pub fn determine_updates(update: &SoftwareUpdate) -> Status {
    let updates_available = update.last_updates_available as usize;
    if !update.automatic_check_enabled && updates_available == 0 {
        check_softwareupdate_output(&softwareupdate_output())
    } else if updates_available == 0 {
        Status::Ok
    } else {
        Status::Warning(updates_available)
    }
}
