use std::mem;
use std::ptr::null;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use coreaudio_sys::{
    kAudioDevicePropertyScopeInput, kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
    kAudioObjectPropertyElementMaster, AudioObjectGetPropertyData, AudioObjectPropertyAddress,
    AudioObjectSetPropertyData, OSStatus, UInt32,
};
use ctrlc;

mod helper;

fn main() -> Result<(), anyhow::Error> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();
    ctrlc::set_handler(move || {
        shutdown_clone.store(true, Ordering::SeqCst);
    })?;

    while !shutdown.load(Ordering::SeqCst) {
        let device_id =
            helper::get_default_input_device_id().expect("Failed to get default input device id");
        let mic = MicDevice::new(device_id);
        let vol = mic.get_volume();
        if vol.is_err() {
            eprintln!("Failed to get mic volume");
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue;
        }

        let vol = vol.unwrap();
        if vol != Volume::MIN_VOLUME {
            let result = mic.set_volume(&Volume::MAX_VOLUME);
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
struct MicDevice {
    device_id: u32,
}

impl MicDevice {
    fn new(device_id: u32) -> Self {
        Self { device_id }
    }

    fn get_volume(&self) -> Result<Volume, anyhow::Error> {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
            mScope: kAudioDevicePropertyScopeInput,
            mElement: kAudioObjectPropertyElementMaster,
        };

        let volume: f32 = 0.0;
        let data_size = mem::size_of::<UInt32>();
        let status: OSStatus = unsafe {
            AudioObjectGetPropertyData(
                self.device_id,
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

    fn set_volume(&self, volume: &Volume) -> Result<(), anyhow::Error> {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
            mScope: kAudioDevicePropertyScopeInput,
            mElement: kAudioObjectPropertyElementMaster,
        };

        let volume: f32 = volume.0;
        let data_size = mem::size_of::<UInt32>();
        let status: OSStatus = unsafe {
            AudioObjectSetPropertyData(
                self.device_id,
                &property_address as *const _,
                0,
                null(),
                data_size as _,
                &volume as *const _ as *mut _,
            )
        };

        if status != 0 {
            return Err(anyhow::anyhow!("Failed to get mic volume"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_mic_volume() {
        let device_id =
            helper::get_default_input_device_id().expect("Failed to get default input device id");
        let mic = MicDevice::new(device_id);
        let volume = Volume::new(0.5);

        mic.set_volume(&volume).expect("Failed to set mic volume");
        let vol = mic.get_volume().expect("Failed to get mic volume");
        assert_eq!(vol, volume);
    }
}
