use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const DETECT_TIMEOUT_SECS: u64 = 60;

pub fn detect_wm_class_for_exec(exec: &str, app_name: &str) -> Result<String, String> {
    ensure_tools_installed()?;

    let binary = exec_binary_path(exec);
    if binary.is_empty() {
        return Err("Desktop launcher has no executable path".to_string());
    }

    let child = Command::new(&binary)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Failed to launch application: {error}"))?;

    let root_pid = child.id();
    let deadline = Instant::now() + Duration::from_secs(DETECT_TIMEOUT_SECS);

    while Instant::now() < deadline {
        for pid in descendant_pids(root_pid) {
            if let Some(class) = wm_class_for_pid(pid) {
                return Ok(class);
            }
        }

        if let Some(class) = wm_class_from_wmctrl(app_name) {
            return Ok(class);
        }

        if let Some(class) = wm_class_for_window_name(app_name) {
            return Ok(class);
        }

        thread::sleep(Duration::from_millis(500));
    }

    Ok(app_name.to_string())
}

fn exec_binary_path(exec: &str) -> String {
    exec.split_whitespace()
        .next()
        .unwrap_or(exec)
        .trim()
        .to_string()
}

fn ensure_tools_installed() -> Result<(), String> {
    if Command::new("xdotool").arg("--version").output().is_err() {
        return Err(
            "xdotool is required for dock grouping detection. Install it with: sudo apt install xdotool"
                .to_string(),
        );
    }
    if Command::new("xprop").arg("-version").output().is_err() {
        return Err(
            "xprop is required for dock grouping detection. Install it with: sudo apt install x11-utils"
                .to_string(),
        );
    }
    Ok(())
}

fn descendant_pids(root_pid: u32) -> Vec<u32> {
    let mut all = vec![root_pid];
    let mut queue = vec![root_pid];

    while let Some(pid) = queue.pop() {
        let output = Command::new("pgrep")
            .args(["-P", &pid.to_string()])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    if let Ok(child_pid) = line.trim().parse::<u32>() {
                        all.push(child_pid);
                        queue.push(child_pid);
                    }
                }
            }
        }
    }

    all
}

fn wm_class_for_pid(pid: u32) -> Option<String> {
    let search = Command::new("xdotool")
        .args(["search", "--pid", &pid.to_string()])
        .output()
        .ok()?;

    if !search.status.success() {
        return None;
    }

    let window_ids = String::from_utf8_lossy(&search.stdout);
    for window_id in window_ids.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(class) = wm_class_for_window(window_id) {
            return Some(class);
        }
    }

    None
}

fn wm_class_for_window_name(app_name: &str) -> Option<String> {
    let output = Command::new("xdotool")
        .args(["search", "--name", app_name])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let window_ids = String::from_utf8_lossy(&output.stdout);
    for window_id in window_ids.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(class) = wm_class_for_window(window_id) {
            return Some(class);
        }
    }

    None
}

fn wm_class_from_wmctrl(app_name: &str) -> Option<String> {
    let output = Command::new("wmctrl").args(["-lx"]).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let needle = app_name.to_lowercase();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let lower = line.to_lowercase();
        if !lower.contains(&needle) {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let wm_class = parts[2];
        if let Some((instance, class)) = wm_class.split_once('.') {
            if !class.is_empty() {
                return Some(class.to_string());
            }
            if !instance.is_empty() {
                return Some(instance.to_string());
            }
        }

        return Some(wm_class.to_string());
    }

    None
}

fn wm_class_for_window(window_id: &str) -> Option<String> {
    let output = Command::new("xprop")
        .args(["-id", window_id, "WM_CLASS"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    parse_wm_class(&String::from_utf8_lossy(&output.stdout))
}

fn parse_wm_class(output: &str) -> Option<String> {
    let values = output.split('=').nth(1)?.trim();
    let quoted: Vec<String> = values
        .split('"')
        .map(str::trim)
        .filter(|part| !part.is_empty() && *part != ",")
        .map(str::to_string)
        .collect();

    match quoted.len() {
        0 => None,
        1 => Some(quoted[0].clone()),
        _ => Some(quoted[1].clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wm_class_pair() {
        let parsed = parse_wm_class(r#"WM_CLASS(STRING) = "lmms", "LMMS""#);
        assert_eq!(parsed, Some("LMMS".to_string()));
    }
}
