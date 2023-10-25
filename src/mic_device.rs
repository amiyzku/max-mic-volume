use anyhow::{ensure, Result};
use coreaudio_sys::{
    kAudioDevicePropertyScopeInput, kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
    kAudioObjectPropertyElementMaster, AudioObjectGetPropertyData, AudioObjectPropertyAddress,
    AudioObjectSetPropertyData, OSStatus, UInt32,
};
use std::mem;
use std::ptr::null;

use crate::volume::Volume;

#[derive(Debug, PartialEq, Eq)]
pub struct MicDevice {
    device_id: u32,
}

impl MicDevice {
    pub fn new(device_id: u32) -> Self {
        Self { device_id }
    }

    pub fn get_volume(&self) -> Result<Volume> {
        let volume: f32 = 0.0;
        let data_size = mem::size_of::<UInt32>();

        let status: OSStatus = unsafe {
            AudioObjectGetPropertyData(
                self.device_id,
                &Self::create_property_address() as *const _,
                0,
                null(),
                &data_size as *const _ as *mut _,
                &volume as *const _ as *mut _,
            )
        };

        ensure!(status == 0, "Failed to get mic volume");

        Ok(Volume::new(volume))
    }

    pub fn set_volume(&self, volume: &Volume) -> Result<()> {
        let volume: f32 = volume.value();
        let data_size = mem::size_of::<UInt32>();

        let status: OSStatus = unsafe {
            AudioObjectSetPropertyData(
                self.device_id,
                &Self::create_property_address() as *const _,
                0,
                null(),
                data_size as _,
                &volume as *const _ as *mut _,
            )
        };

        ensure!(status == 0, "Failed to set mic volume");

        Ok(())
    }

    const fn create_property_address() -> AudioObjectPropertyAddress {
        AudioObjectPropertyAddress {
            mSelector: kAudioHardwareServiceDeviceProperty_VirtualMasterVolume,
            mScope: kAudioDevicePropertyScopeInput,
            mElement: kAudioObjectPropertyElementMaster,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper;

    #[test]
    fn get_the_correct_volume() {
        // Arrange
        let device_id =
            helper::get_default_input_device_id().expect("Failed to get default input device id");
        let sut = MicDevice::new(device_id);
        let expected = Volume::new(0.5);
        sut.set_volume(&expected).expect("Failed to set mic volume");

        // Act
        let vol = sut.get_volume().expect("Failed to get mic volume");

        // Assert
        assert_eq!(vol, expected);
    }

    #[test]
    fn set_the_correct_volume() {
        // Arrange
        let device_id =
            helper::get_default_input_device_id().expect("Failed to get default input device id");
        let sut = MicDevice::new(device_id);
        let volume = Volume::new(0.5);

        // Act
        let result = sut.set_volume(&volume);

        // Assert
        assert!(result.is_ok());
    }
}
