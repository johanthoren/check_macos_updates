pub mod plists {
    pub const NO_UPDATES: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>AutomaticCheckEnabled</key>
	<true/>
	<key>LastSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastAttemptSystemVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>LastUpdatesAvailable</key>
	<integer>0</integer>
	<key>LastRecommendedUpdatesAvailable</key>
	<integer>0</integer>
	<key>LastAttemptBuildVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>RecommendedUpdates</key>
	<array/>
	<key>LastFullSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastRecommendedMajorOSBundleIdentifier</key>
	<string></string>
	<key>PrimaryLanguages</key>
	<array>
		<string>en</string>
		<string>en-SE</string>
		<string>en-GB</string>
	</array>
	<key>LastSessionSuccessful</key>
	<true/>
	<key>LastBackgroundSuccessfulDate</key>
	<date>2023-05-04T21:40:26Z</date>
	<key>LastResultCode</key>
	<integer>2</integer>
</dict>
</plist>
"#;

    pub const ONE_UPDATE: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>AutomaticCheckEnabled</key>
	<true/>
	<key>LastSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastAttemptSystemVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>LastUpdatesAvailable</key>
	<integer>1</integer>
	<key>LastRecommendedUpdatesAvailable</key>
	<integer>1</integer>
	<key>LastAttemptBuildVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>RecommendedUpdates</key>
	<array/>
	<key>LastFullSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastRecommendedMajorOSBundleIdentifier</key>
	<string></string>
	<key>PrimaryLanguages</key>
	<array>
		<string>en</string>
		<string>en-SE</string>
		<string>en-GB</string>
	</array>
	<key>LastSessionSuccessful</key>
	<true/>
	<key>LastBackgroundSuccessfulDate</key>
	<date>2023-05-04T21:40:26Z</date>
	<key>LastResultCode</key>
	<integer>2</integer>
</dict>
</plist>
"#;

    pub const ONE_UPDATE_NO_AUTO_CHECK: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>LastSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastAttemptSystemVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>LastUpdatesAvailable</key>
	<integer>1</integer>
	<key>LastRecommendedUpdatesAvailable</key>
	<integer>1</integer>
	<key>LastAttemptBuildVersion</key>
	<string>13.3.1 (22E772610a)</string>
	<key>RecommendedUpdates</key>
	<array/>
	<key>LastFullSuccessfulDate</key>
	<date>2023-05-04T21:40:12Z</date>
	<key>LastRecommendedMajorOSBundleIdentifier</key>
	<string></string>
	<key>PrimaryLanguages</key>
	<array>
		<string>en</string>
		<string>en-SE</string>
		<string>en-GB</string>
	</array>
	<key>LastSessionSuccessful</key>
	<true/>
	<key>LastBackgroundSuccessfulDate</key>
	<date>2023-05-04T21:40:26Z</date>
	<key>LastResultCode</key>
	<integer>2</integer>
</dict>
</plist>
"#;
}
