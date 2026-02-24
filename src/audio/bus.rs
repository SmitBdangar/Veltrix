//! Audio buses for grouping sounds (Master, SFX, Music).

/// Predefined audio buses for volume control routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioBusName {
    Master,
    Music,
    Sfx,
    UI,
}

/// A bus controlling the volume multiplier for a group of sounds.
#[derive(Debug, Clone)]
pub struct AudioBus {
    pub name: AudioBusName,
    pub volume: f32,
}

impl AudioBus {
    pub fn new(name: AudioBusName) -> Self {
        Self { name, volume: 1.0 }
    }
}
