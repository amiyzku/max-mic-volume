#[derive(Debug, PartialEq, PartialOrd)]
pub struct Volume(f32);

impl Volume {
    const MIN: f32 = 0.0;
    const MAX: f32 = 1.0;
    pub const MIN_VOLUME: Self = Self(Self::MIN);
    pub const MAX_VOLUME: Self = Self(Self::MAX);

    pub fn new(volume: f32) -> Self {
        Self(volume.min(Self::MAX).max(Self::MIN))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn is_mute(&self) -> bool {
        self == &Self::MIN_VOLUME
    }

    pub fn is_max(&self) -> bool {
        self == &Self::MAX_VOLUME
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_create_a_volume_greater_than_max() {
        // Arrange
        let volume = 1.1;

        // Act
        let sut = Volume::new(volume);

        // Assert
        assert_eq!(sut, Volume::MAX_VOLUME);
    }

    #[test]
    fn cannot_create_a_volume_less_than_min() {
        // Arrange
        let volume = -0.1;

        // Act
        let sut = Volume::new(volume);

        // Assert
        assert_eq!(sut, Volume::MIN_VOLUME);
    }

    #[test]
    fn can_create_a_volume_between_min_and_max() {
        // Arrange
        let volume = 0.5;

        // Act
        let sut = Volume::new(volume);

        // Assert
        assert_eq!(sut, Volume(volume));
    }

    #[test]
    fn can_get_the_value_of_a_volume() {
        // Arrange
        let volume = 0.5;
        let sut = Volume::new(volume);

        // Act
        let result = sut.value();

        // Assert
        assert_eq!(result, volume);
    }

    #[test]
    fn can_check_if_a_volume_is_mute() {
        // Arrange
        let volume = 0.0;
        let sut = Volume::new(volume);

        // Act
        let result = sut.is_mute();

        // Assert
        assert!(result);
    }

    #[test]
    fn can_check_if_a_volume_is_max() {
        // Arrange
        let volume = 1.0;
        let sut = Volume::new(volume);

        // Act
        let result = sut.is_max();

        // Assert
        assert!(result);
    }
}
