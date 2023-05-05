#[cfg(test)]
mod tests {
    use check_macos_updates::check_updates;
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
    fn test_with_no_available_updates() {
        let output = mock_output(
            b"Software Update Tool\n\nFinding available software\n".to_vec(),
            b"No new software available.\n".to_vec(),
            0,
        );

        let result = Ok(output);

        let (message, exit_code) = check_updates(result);
        assert_eq!(message, "OK - No updates available");
        assert_eq!(exit_code, 0);
    }
}
