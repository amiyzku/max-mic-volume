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
        if vol != Volume::MIN_VOLUME {
            let result = Mic::set_volume(&Volume::MAX_VOLUME);
            if result.is_err() {
                eprintln!("Failed to set mic volume");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    std::process::exit(0);
}

#[derive(Debug)]
struct Volume(u8);

impl Volume {
    const MIN: u8 = 0;
    const MAX: u8 = 100;
    const MIN_VOLUME: Self = Self(Self::MIN);
    const MAX_VOLUME: Self = Self(Self::MAX);

    fn new(volume: u8) -> Self {
        Self(volume.min(Self::MAX).max(Self::MIN))
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
struct Mic {}

impl Mic {
    fn get_volume() -> Result<Volume, anyhow::Error> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg("input volume of (get volume settings)")
            .output()?;

        let vol = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u8>()?;

        Ok(Volume::new(vol))
    }

    fn set_volume(volume: &Volume) -> Result<(), anyhow::Error> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!("set volume input volume {}", volume.0))
            .output()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_mic_volume() {
        let volume = Volume::new(50);
        Mic::set_volume(&volume).expect("Failed to set mic volume");
        let vol = Mic::get_volume().expect("Failed to get mic volume");
        assert_eq!(vol, volume);
    }
}
