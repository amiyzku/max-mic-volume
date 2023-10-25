use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use ctrlc;

mod helper;
mod mic_device;
mod volume;

use mic_device::MicDevice;
use volume::Volume;

fn main() -> Result<()> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    while !shutdown.load(Ordering::SeqCst) {
        let device_id = helper::get_default_input_device_id();
        if device_id.is_err() {
            eprintln!("Failed to get default input device id");
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        let mic = MicDevice::new(device_id.unwrap());
        let vol = mic.volume();
        if vol.is_err() {
            eprintln!("Failed to get mic volume");
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        let vol = vol.unwrap();
        if vol.is_mute() || vol.is_max() {
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        if mic.set_volume(&Volume::MAX_VOLUME).is_err() {
            eprintln!("Failed to set mic volume");
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    std::process::exit(0);
}
