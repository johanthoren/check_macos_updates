use nagios_range::NagiosRange;
use serde::Deserialize;
use std::fmt;
use std::process::{self, Output};

pub const PLIST_FILE: &str = "/Library/Preferences/com.apple.SoftwareUpdate.plist";

#[derive(Clone, Debug, PartialEq)]
pub struct Thresholds {
    pub warning: Option<NagiosRange>,
    pub critical: Option<NagiosRange>,
}

#[derive(Debug, PartialEq)]
pub enum UnkownVariant {
    NotMacOS,
    NoThresholds,
    RangeParseError(String, nagios_range::Error),
    UnableToDetermineUpdates,
    UnableToParsePlist,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok(usize),
    Warning(usize),
    Critical(usize),
    Unknown(UnkownVariant),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Ok(n) => write!(f, "OK - {} updates available|'Available Updates'={}", n, n),
            Status::Warning(n) => write!(
                f,
                "WARNING - Updates available: {}|'Available Updates'={}",
                n, n
            ),
            Status::Critical(n) => write!(
                f,
                "CRITICAL - Updates available: {}|'Available Updates'={}",
                n, n
            ),
            Status::Unknown(UnkownVariant::NotMacOS) => {
                write!(f, "UNKNOWN - Not running on macOS")
            }
            Status::Unknown(UnkownVariant::NoThresholds) => {
                write!(f, "UNKNOWN - No thresholds provided")
            }
            Status::Unknown(UnkownVariant::RangeParseError(s, e)) => {
                write!(
                    f,
                    "UNKNOWN - Unable to parse range '{}' with error: {}",
                    s, e
                )
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
            Status::Ok(_) => 0,
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

pub fn check_softwareupdate_output(
    output: &Result<Output, std::io::Error>,
    thresholds: Thresholds,
) -> Status {
    match output {
        Ok(output) => {
            let output_stderr = String::from_utf8_lossy(&output.stderr);
            let output_stdout = String::from_utf8_lossy(&output.stdout);

            let n: usize = if output_stderr.contains("No new software available.") {
                0
            } else {
                output_stdout
                    .lines()
                    .filter(|l| l.contains("* Label:"))
                    .count()
            };

            if let Some(c) = thresholds.critical {
                if c.check(n as f64) {
                    return Status::Critical(n);
                }
            }

            if let Some(w) = thresholds.warning {
                if w.check(n as f64) {
                    return Status::Warning(n);
                }
            }

            Status::Ok(n)
        }
        Err(_) => Status::Unknown(UnkownVariant::UnableToDetermineUpdates),
    }
}

pub fn determine_updates(update: &SoftwareUpdate, thresholds: Thresholds) -> Status {
    let n = update.last_updates_available as usize;
    if !update.automatic_check_enabled && n == 0 {
        check_softwareupdate_output(&softwareupdate_output(), thresholds)
    } else {
        if let Some(c) = thresholds.critical {
            if c.check(n as f64) {
                return Status::Critical(n);
            }
        }

        if let Some(w) = thresholds.warning {
            if w.check(n as f64) {
                return Status::Warning(n);
            }
        }

        Status::Ok(n)
    }
}
