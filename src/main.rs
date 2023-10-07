use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use ctrlc;

fn main() -> Result<(), anyhow::Error> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    while !shutdown.load(Ordering::SeqCst) {
        let vol = Mic::get_volume();
        if vol.is_err() {
            eprintln!("Failed to get mic volume");
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        let vol = vol.unwrap();
        if vol != 0 {
            let result = Mic::set_volume(100);
            if result.is_err() {
                eprintln!("Failed to set mic volume");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    std::process::exit(0);
}

struct Mic {}

impl Mic {
    fn get_volume() -> Result<u8, anyhow::Error> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg("input volume of (get volume settings)")
            .output()?;

        let vol = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u8>()?;

        Ok(vol)
    }

    fn set_volume(volume: u8) -> Result<(), anyhow::Error> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!("set volume input volume {}", volume))
            .output()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_mic_volume() {
        Mic::set_volume(50).expect("Failed to set mic volume");
        let vol = Mic::get_volume().expect("Failed to get mic volume");
        assert_eq!(vol, 50);
    }
}
