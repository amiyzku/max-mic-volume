use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use ctrlc;

mod cli;
mod helper;
mod mic_device;
mod volume;

use cli::Cli;
use mic_device::MicDevice;
use volume::Volume;

fn main() -> Result<()> {
    let args = Cli::parse();
    let polling_interval_ms = args.polling_interval_ms;

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    while !shutdown.load(Ordering::SeqCst) {
        let device_id = helper::get_default_input_device_id();
        if device_id.is_err() {
            eprintln!("Failed to get default input device id");
            sleep(Duration::from_millis(polling_interval_ms));
            continue;
        }

        let mic = MicDevice::new(device_id.unwrap());
        let vol = mic.volume();
        if vol.is_err() {
            eprintln!("Failed to get mic volume");
            sleep(Duration::from_millis(polling_interval_ms));
            continue;
        }

        let vol = vol.unwrap();
        if vol.is_mute() || vol.is_max() {
            sleep(Duration::from_millis(polling_interval_ms));
            continue;
        }

        if mic.set_volume(&Volume::MAX_VOLUME).is_err() {
            eprintln!("Failed to set mic volume");
            sleep(Duration::from_millis(polling_interval_ms));
            continue;
        }

        sleep(Duration::from_millis(polling_interval_ms));
    }

    std::process::exit(0);
}
