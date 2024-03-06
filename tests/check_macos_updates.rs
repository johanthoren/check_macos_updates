#[cfg(test)]
mod plist_examples;
mod tests {
    use crate::plist_examples::plists::{NO_UPDATES, ONE_UPDATE, ONE_UPDATE_NO_AUTO_CHECK};
    use check_macos_updates::*;
    use nagios_range::NagiosRange;
    use pretty_assertions::assert_eq;
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

        let thresholds = Thresholds {
            warning: Some(NagiosRange::from("1").unwrap()),
            critical: Some(NagiosRange::from("2").unwrap()),
        };

        let status = check_softwareupdate_output(&result, thresholds);
        assert_eq!(
            status.to_string(),
            "OK - 0 updates available|'Available Updates'=0".to_string()
        );
        assert_eq!(status.to_int(), 0);
    }

    #[test]
    fn test_manual_check_with_2_available_updates() {
        let output = mock_output(
            r#"Software Update Tool

Finding available software
Software Update found the following new or updated software:
   * Label: Security Update 2022-001 (Catalina)
       Title: Security Update 2022-001 (Catalina), 1 GB
    Version: 10.15.7
   * Label: Safari15.0CatalinaAuto-15.0
       Title: Safari15.0CatalinaAuto-15.0, 1 MB
    Version: 15.0
"#
            .as_bytes()
            .to_vec(),
            Vec::new(),
            0,
        );

        let result = Ok(output);

        let thresholds = Thresholds {
            warning: Some(NagiosRange::from("1").unwrap()),
            critical: Some(NagiosRange::from("2").unwrap()),
        };

        let status = check_softwareupdate_output(&result, thresholds);
        assert_eq!(
            status.to_string(),
            "WARNING - Updates available: 2|'Available Updates'=2".to_string()
        );
    }

    #[test]
    fn test_read_plist_with_no_available_updates() {
        let software_update_plist: SoftwareUpdate =
            plist::from_bytes(NO_UPDATES.as_bytes()).expect("Failed to parse plist");

        println!("{:?}", software_update_plist);

        let thresholds = Thresholds {
            warning: Some(NagiosRange::from("1").unwrap()),
            critical: Some(NagiosRange::from("2").unwrap()),
        };

        assert_eq!(software_update_plist.last_updates_available, 0);
        assert_eq!(
            determine_updates(&software_update_plist, thresholds.clone()),
            Status::Ok(0)
        );

        let status = determine_updates(&software_update_plist, thresholds);
        assert_eq!(
            status.to_string(),
            "OK - 0 updates available|'Available Updates'=0".to_string()
        );
        assert_eq!(status.to_int(), 0);
    }

    #[test]
    fn test_read_plist_with_one_available_update() {
        let software_update_plist: SoftwareUpdate =
            plist::from_bytes(ONE_UPDATE.as_bytes()).expect("Failed to parse plist");

        println!("{:?}", software_update_plist);

        let thresholds = Thresholds {
            warning: Some(NagiosRange::from("0").unwrap()),
            critical: Some(NagiosRange::from("3").unwrap()),
        };

        assert_eq!(software_update_plist.last_updates_available, 1);
        assert_eq!(
            determine_updates(&software_update_plist, thresholds.clone()),
            Status::Warning(1)
        );

        let status = determine_updates(&software_update_plist, thresholds);
        assert_eq!(
            status.to_string(),
            "WARNING - Updates available: 1|'Available Updates'=1".to_string()
        );
        assert_eq!(status.to_int(), 1);
    }

    #[test]
    fn test_read_plist_with_one_available_update_but_no_auto_check() {
        let software_update_plist: SoftwareUpdate =
            plist::from_bytes(ONE_UPDATE_NO_AUTO_CHECK.as_bytes()).expect("Failed to parse plist");

        println!("{:?}", software_update_plist);

        let thresholds = Thresholds {
            warning: Some(NagiosRange::from("0").unwrap()),
            critical: Some(NagiosRange::from("3").unwrap()),
        };

        assert_eq!(software_update_plist.last_updates_available, 1);
        assert_eq!(
            determine_updates(&software_update_plist, thresholds.clone()),
            Status::Warning(1)
        );

        let status = determine_updates(&software_update_plist, thresholds);
        assert_eq!(
            status.to_string(),
            "WARNING - Updates available: 1|'Available Updates'=1".to_string()
        );
        assert_eq!(status.to_int(), 1);
    }
}
