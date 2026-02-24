//! The rodio-based audio manager.

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;

use super::{bus::{AudioBus, AudioBusName}, clip::AudioClip};
use crate::assets::server::AssetServer;
use crate::assets::handle::Handle;

/// The central audio playback system.
pub struct AudioManager {
    // Keep the stream alive to prevent audio dropping.
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    
    /// Global buses for volume routing.
    pub buses: HashMap<AudioBusName, AudioBus>,
    
    /// Active playback sinks.
    sinks: Vec<Sink>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    /// Initialize the audio subsystem.
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default()
            .unwrap_or_else(|e| panic!("Failed to initialize audio stream: {:?}", e));

        let mut buses = HashMap::new();
        buses.insert(AudioBusName::Master, AudioBus::new(AudioBusName::Master));
        buses.insert(AudioBusName::Music, AudioBus::new(AudioBusName::Music));
        buses.insert(AudioBusName::Sfx, AudioBus::new(AudioBusName::Sfx));
        buses.insert(AudioBusName::UI, AudioBus::new(AudioBusName::UI));

        Self {
            _stream,
            stream_handle,
            buses,
            sinks: Vec::new(),
        }
    }

    /// Play an audio clip once on a specific bus.
    pub fn play(
        &mut self,
        assets: &AssetServer,
        clip_handle: &Handle<AudioClip>,
        bus_name: AudioBusName,
        volume: f32,
        pitch: f32,
    ) {
        if let Some(clip) = assets.with_asset(clip_handle, |c| c.clone()) {
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                // Calculate effective volume: clip volume * bus volume * master volume
                let bus_vol = self.buses.get(&bus_name).map(|b| b.volume).unwrap_or(1.0);
                let master_vol = self.buses.get(&AudioBusName::Master).map(|b| b.volume).unwrap_or(1.0);
                
                sink.set_volume(volume * bus_vol * master_vol);
                sink.set_speed(pitch);
                
                sink.append(clip.decoder());
                // Detach sink so it cleans itself up when finished
                sink.detach();
            }
        }
    }

    /// Play an audio clip looping on a specific bus.
    ///
    /// Returns the active `Sink` so you can stop it later.
    pub fn play_looped(
        &mut self,
        assets: &AssetServer,
        clip_handle: &Handle<AudioClip>,
        bus_name: AudioBusName,
        volume: f32,
    ) -> Option<Sink> {
        if let Some(clip) = assets.with_asset(clip_handle, |c| c.clone()) {
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                let bus_vol = self.buses.get(&bus_name).map(|b| b.volume).unwrap_or(1.0);
                let master_vol = self.buses.get(&AudioBusName::Master).map(|b| b.volume).unwrap_or(1.0);
                
                sink.set_volume(volume * bus_vol * master_vol);
                
                // Using .repeat_infinite() on the source decoder
                sink.append(clip.decoder().repeat_infinite());
                
                return Some(sink);
            }
        }
        None
    }
}
