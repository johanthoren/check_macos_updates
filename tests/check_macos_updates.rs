#[cfg(test)]
mod plist_examples;
mod tests {
    use crate::plist_examples::plists::{NO_UPDATES, ONE_UPDATE};
    use check_macos_updates::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::Output;

    fn mock_output(stdout: Vec<u8>, stderr: Vec<u8>, status: i32) -> Output {
        Output {
            stdout,
            stderr,
            status: std::process::ExitStatus::from_raw(status),
        }
    }

    #[test]
    fn test_mock_output() {
        let output = mock_output(b"stdout".to_vec(), b"stderr".to_vec(), 0);
        assert_eq!(output.stdout, b"stdout");
        assert_eq!(output.stderr, b"stderr");
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_manual_check_with_no_available_updates() {
        let output = mock_output(
            b"Software Update Tool\n\nFinding available software\n".to_vec(),
            b"No new software available.\n".to_vec(),
            0,
        );

        let result = Ok(output);

        let status = check_output(&result);
        assert_eq!(status.to_string(), "OK - No updates available".to_string());
        assert_eq!(status.to_int(), 0);
    }

    #[test]
    fn test_read_plist_with_no_available_updates() {
        let software_update_plist: SoftwareUpdate =
            plist::from_bytes(NO_UPDATES.as_bytes()).expect("Failed to parse plist");

        println!("{:?}", software_update_plist);

        assert_eq!(software_update_plist.last_updates_available, 0);
        assert_eq!(determine_updates(&software_update_plist), Status::Ok);

        let status = determine_updates(&software_update_plist);
        assert_eq!(status.to_string(), "OK - No updates available".to_string());
        assert_eq!(status.to_int(), 0);
    }

    #[test]
    fn test_read_plist_with_one_available_update() {
        let software_update_plist: SoftwareUpdate =
            plist::from_bytes(ONE_UPDATE.as_bytes()).expect("Failed to parse plist");

        println!("{:?}", software_update_plist);

        assert_eq!(software_update_plist.last_updates_available, 1);
        assert_eq!(determine_updates(&software_update_plist), Status::Warning);

        let status = determine_updates(&software_update_plist);
        assert_eq!(
            status.to_string(),
            "WARNING - Updates available".to_string()
        );
        assert_eq!(status.to_int(), 1);
    }
}
