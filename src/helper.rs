use anyhow::{ensure, Result};
use std::{mem, ptr::null};

use coreaudio_sys::{
    kAudioHardwarePropertyDefaultInputDevice, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject, AudioObjectGetPropertyData,
    AudioObjectPropertyAddress, OSStatus,
};

pub fn get_default_input_device_id() -> Result<u32> {
    let mut device_id: u32 = 0;
    let data_size = mem::size_of::<u32>();

    let status: OSStatus = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &create_property_address() as *const _,
            0,
            null(),
            &data_size as *const _ as *mut _,
            &mut device_id as *mut _ as *mut _,
        )
    };

    ensure!(status == 0, "Failed to get default input device");

    Ok(device_id)
}

const fn create_property_address() -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultInputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_input_device_id() {
        // Act
        let sut = get_default_input_device_id();

        // Assert
        assert!(sut.is_ok());
    }
}
