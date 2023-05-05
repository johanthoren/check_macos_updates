use std::process::{self, Output};

pub fn get_output() -> Result<Output, std::io::Error> {
    process::Command::new("softwareupdate").arg("-l").output()
}

pub fn check_updates(output: Result<Output, std::io::Error>) -> (String, i32) {
    match output {
        Ok(output) => {
            let output_stderr = String::from_utf8_lossy(&output.stderr);

            if output_stderr.contains("No new software available.") {
                return (String::from("OK - No updates available"), 0);
            } else {
                return (String::from("WARNING - Updates available"), 1);
            }
        }
        Err(_) => {
            return (String::from("UNKNOWN - Error running softwareupdate"), 3);
        }
    }
}
