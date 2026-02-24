//! Audio engine using rodio: AudioManager, AudioClip, spatial audio, and buses.

pub mod bus;
pub mod clip;
pub mod manager;
pub mod spatial;

pub use manager::AudioManager;
