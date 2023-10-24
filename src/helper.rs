use anyhow::Result;
use std::{mem, ptr::null};

use coreaudio_sys::{
    kAudioHardwarePropertyDefaultInputDevice, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject, AudioObjectGetPropertyData,
    AudioObjectPropertyAddress, OSStatus,
};

pub fn get_default_input_device_id() -> Result<u32> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultInputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let mut device_id: u32 = 0;
    let data_size = mem::size_of::<u32>();
    let status: OSStatus = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &property_address as *const _,
            0,
            null(),
            &data_size as *const _ as *mut _,
            &mut device_id as *mut _ as *mut _,
        )
    };

    if status != 0 {
        return Err(anyhow::anyhow!("Failed to get default input device"));
    }

    Ok(device_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_input_device_id() {
        // Act
        let device_id = get_default_input_device_id();

        // Assert
        assert!(device_id.is_ok());
    }
}
