#[derive(Debug)]
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
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
        let result = Volume::new(volume);

        // Assert
        assert_eq!(result, Volume::MAX_VOLUME);
    }

    #[test]
    fn cannot_create_a_volume_less_than_min() {
        // Arrange
        let volume = -0.1;

        // Act
        let result = Volume::new(volume);

        // Assert
        assert_eq!(result, Volume::MIN_VOLUME);
    }
}
