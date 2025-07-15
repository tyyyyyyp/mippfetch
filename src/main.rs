use std::{env, process::Command, path::PathBuf};
use configparser::ini::Ini;
use sysinfo::{System, SystemExt};

fn get_gpu_info() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -iE 'vga|3d'")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let info = String::from_utf8_lossy(&output.stdout);
    let line = info.lines().next()?.trim();

    // Extract text after first colon
    let desc = line.splitn(2, ':').nth(1)?.trim();

    let keywords = ["gtx", "rtx", "rx", "radeon", "intel"];

    for &kw in &keywords {
        if let Some(start) = desc.to_lowercase().find(kw) {
            let substr = &desc[start..];
            let words: Vec<&str> = substr.split_whitespace().take(3).collect();

            let mut model = words.join(" ");

            // Remove anything after first closing bracket, parenthesis, or extra punctuation
            if let Some(pos) = model.find(|c| c == ']' || c == '(' || c == ',') {
                model = model[..pos].to_string();
            }

            return Some(model.trim().to_string());
        }
    }

    Some(desc.to_string())
}


fn get_config_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(format!("{}/.config/mippfetch/mippfetch.conf", home))
}

fn load_config() -> Ini {
    let mut conf = Ini::new();
    let _ = conf.load(get_config_path());
    conf
}

fn get_ascii_banner(distro: &str) -> Vec<&'static str> {
    match distro.to_lowercase().as_str() {
        "arch" => vec![
            r"       /\        ",
            r"      /  \       ",
            r"     /    \      ",
            r"    /      \     ",
            r"   /        \    ",
            r"  /   ,--,   \   ",
            r" /   |    |   \  ",
            r"/_-''      ''-_\ ",
        ],
        "ubuntu" => vec![
            "   .--.   ",
            " .'    '. ",
            ":  o  o  :",
            ":        :",
            ":  \\__/  :",
            " '.    .' ",
            "   '--'ubuntu!",
        ],        _ => vec![r"Unknown distro"],
    }
}

fn get_wm() -> Option<String> {
    let env_vars = ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "WAYLAND_DISPLAY", "XDG_SESSION_TYPE"];
    for var in env_vars {
        if let Ok(val) = env::var(var) {
            return Some(val);
        }
    }

    let output = Command::new("ps")
        .arg("-e")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())?;

    let wms = ["i3", "hyprland", "bspwm", "openbox", "xmonad", "awesome", "herbstluftwm", "dwm", "kwin", "xfwm4", "mutter"];
    for wm in wms {
        if output.to_lowercase().contains(wm) {
            return Some(wm.to_string());fn get_gpu_info() -> Option<String> {
    // Use "lspci" to get GPU info — filters for VGA or 3D controllers
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -iE 'vga|3d'")
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let info = String::from_utf8_lossy(&output.stdout);
    let gpu_info = info.lines().next()?.trim().to_string(); // Take first matching line
    
    Some(gpu_info)
}
        }
    }

    None
}

fn get_shell() -> Option<String> {
    env::var("SHELL").ok().map(|s| s.split('/').last().unwrap_or("").to_string())
}

fn get_pkg_count(manager: &str) -> Option<usize> {
    let cmd_output = match manager {
        "pacman" => Command::new("sh").arg("-c").arg("pacman -Q | wc -l").output(),
        "dpkg" => Command::new("sh").arg("-c").arg("dpkg -l | wc -l").output(),
        _ => return None,
    }.ok()?;

    let output = String::from_utf8(cmd_output.stdout).ok()?;
    output.trim().parse().ok()
}

fn print_fetch() {
    let conf = load_config();
    let sys = System::new_all();

    let banner_enabled = conf.getbool("banner", "enabled").unwrap_or(Some(true)).unwrap_or(true);
    let distro = conf.get("banner", "distro").unwrap_or("arch".to_string());
    let manager = conf.get("package", "manager").unwrap_or("auto".to_string());

    let use_mem = conf.getbool("features", "memory").unwrap_or(Some(true)).unwrap_or(true);
    let use_gpu = conf.getbool("features", "gpu").unwrap_or(Some(true)).unwrap_or(true);
    let use_shell = conf.getbool("features", "shell").unwrap_or(Some(true)).unwrap_or(true);
    let use_wm = conf.getbool("features", "wm").unwrap_or(Some(true)).unwrap_or(true);
    let use_term = conf.getbool("features", "terminal").unwrap_or(Some(true)).unwrap_or(true);
    let use_pkg = conf.getbool("features", "packages").unwrap_or(Some(true)).unwrap_or(true);

    // 🔥 Move this up
    let mut info_lines = Vec::new();

    // ✅ New toggles
    let show_user = conf.getbool("features", "username").unwrap_or(Some(true)).unwrap_or(true);
    let show_host = conf.getbool("features", "hostname").unwrap_or(Some(true)).unwrap_or(true);


  
    if show_user {
        if let Ok(user) = env::var("USER") {
            info_lines.push(format!("User: {}", user));
        }
    }

    if show_host {
        info_lines.push(format!("Host: {}", env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())));
    }


    let banner = if banner_enabled { get_ascii_banner(&distro) } else { vec![] };

    let mut info_lines = Vec::new();

    if let Some(user) = env::var("USER").ok() {
        info_lines.push(format!("User: {}", user));
    }

    info_lines.push(format!("Host: {}", env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())));

    if use_shell {
        info_lines.push(format!("Shell: {}", get_shell().unwrap_or_else(|| "unknown".to_string())));
    }

    if use_term {
        info_lines.push(format!("Terminal: {}", env::var("TERM").unwrap_or_else(|_| "unknown".to_string())));
    }

    if use_wm {
        info_lines.push(format!("WM: {}", get_wm().unwrap_or_else(|| "unknown".to_string())));
    }

    if use_pkg {
        let detected_mgr = if manager == "auto" {
            if PathBuf::from("/bin/pacman").exists() {
                "pacman"
            } else if PathBuf::from("/usr/bin/dpkg").exists() {
                "dpkg"
            } else {
                "unknown"
            }
        } else {
            &manager
        };
        let count = get_pkg_count(detected_mgr).unwrap_or(0);
        info_lines.push(format!("Packages ({}): {}", detected_mgr, count));
    }

    if use_mem {
        let total = sys.total_memory() / 1024;
        let used = (sys.total_memory() - sys.available_memory()) / 1024;
        info_lines.push(format!("Memory: {}MB / {}MB", used, total));
    }

    if use_gpu {
    let gpu_info = get_gpu_info().unwrap_or_else(|| "GPU info not available".to_string());
    info_lines.push(format!("GPU: {}", gpu_info));
}

    // Align banner + info side-by-side
    let max_lines = std::cmp::max(banner.len(), info_lines.len());
    for i in 0..max_lines {
        let b = banner.get(i).unwrap_or(&"");
        let i = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        println!("{:<25}{}", b, i);
    }
}

fn main() {
    print_fetch();
}


