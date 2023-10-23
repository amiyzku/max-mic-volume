use std::mem;
use std::process::Command;
use std::ptr::null;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use coreaudio::audio_unit::macos_helpers::get_audio_device_ids_for_scope;
use coreaudio_sys::{
    kAudioDevicePropertyDeviceNameCFString, kAudioDevicePropertyMute,
    kAudioDevicePropertyScopeInput, kAudioDevicePropertyScopeOutput, kAudioHardwareNoError,
    kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
    kAudioHardwareServiceDeviceProperty_VirtualMasterVolume, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject, kCFStringEncodingUTF8,
    AudioDeviceID, AudioHardwareServiceGetPropertyData, AudioObjectGetPropertyData,
    AudioObjectPropertyAddress, AudioObjectSetPropertyData, CFStringGetCString, CFStringRef,
    OSStatus, UInt32,
};
use ctrlc;

fn main() -> Result<(), anyhow::Error> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    while !shutdown.load(Ordering::SeqCst) {
        let vol = Mic::get_volume(54);
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
struct Volume(f32);

impl Volume {
    const MIN: f32 = 0.0;
    const MAX: f32 = 1.0;
    const MIN_VOLUME: Self = Self(Self::MIN);
    const MAX_VOLUME: Self = Self(Self::MAX);

    fn new(volume: f32) -> Self {
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
    fn get_volume(device_id: u32) -> Result<Volume, anyhow::Error> {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
            mScope: kAudioDevicePropertyScopeInput,
            mElement: kAudioObjectPropertyElementMaster,
        };

        let volume: f32 = 0.0;
        let data_size = mem::size_of::<UInt32>();
        let status: OSStatus = unsafe {
            AudioObjectGetPropertyData(
                device_id,
                &property_address as *const _,
                0,
                null(),
                &data_size as *const _ as *mut _,
                &volume as *const _ as *mut _,
            )
        };

        if status != 0 {
            return Err(anyhow::anyhow!("Failed to get mic volume"));
        }

        Ok(Volume::new(volume))
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
        let volume = Volume::new(0.5);
        Mic::set_volume(&volume).expect("Failed to set mic volume");
        let vol = Mic::get_volume(54).expect("Failed to get mic volume");
        assert_eq!(vol, volume);
    }
}
