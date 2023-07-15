use serde::Deserialize;
use std::fmt;
use std::process::{self, Output};

pub const PLIST_FILE: &str = "/Library/Preferences/com.apple.SoftwareUpdate.plist";

#[derive(Debug, PartialEq)]
pub enum UnkownVariant {
    NotMacOS,
    UnableToDetermineUpdates,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Warning,
    Critical,
    Unknown(UnkownVariant),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Status::Ok => "OK - No updates available",
            Status::Warning => "WARNING - Updates available",
            Status::Critical => "CRITICAL - Updates available",
            Status::Unknown(UnkownVariant::NotMacOS) => "UNKNOWN - Not running on macOS",
            Status::Unknown(UnkownVariant::UnableToDetermineUpdates) => {
                "UNKNOWN - Unable to determine available updates"
            }
        };

        write!(f, "{}", s)
    }
}

impl Status {
    pub fn to_int(&self) -> i32 {
        match self {
            Status::Ok => 0,
            Status::Warning => 1,
            Status::Critical => 2,
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
    #[serde(default)]
    pub automatic_download: bool,
    pub last_successful_date: String,
    pub last_attempt_system_version: String,
    pub last_updates_available: u8,
    pub last_recommended_updates_available: u8,
    pub last_attempt_build_version: String,
    pub recommended_updates: Vec<String>,
    pub last_full_successful_date: String,
    pub primary_languages: Vec<String>,
    pub last_session_successful: bool,
    pub last_background_successful_date: String,
    pub last_result_code: u8,
}

pub fn softwareupdate_output() -> Result<Output, std::io::Error> {
    process::Command::new("softwareupdate").arg("-l").output()
}

pub fn check_softwareupdate_output(output: &Result<Output, std::io::Error>) -> Status {
    match output {
        Ok(output) => {
            let output_stderr = String::from_utf8_lossy(&output.stderr);

            if output_stderr.contains("No new software available.") {
                Status::Ok
            } else {
                Status::Warning
            }
        }
        Err(_) => Status::Unknown(UnkownVariant::UnableToDetermineUpdates),
    }
}

pub fn determine_updates(update: &SoftwareUpdate) -> Status {
    if !update.automatic_check_enabled && update.last_updates_available == 0 {
        check_softwareupdate_output(&softwareupdate_output())
    } else if update.last_updates_available == 0 {
        Status::Ok
    } else {
        Status::Warning
    }
}
