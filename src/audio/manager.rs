//! The rodio-based audio manager.

use rodio::{OutputStream, Sink, Source};
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread;

use super::{bus::{AudioBus, AudioBusName}, clip::AudioClip};
use crate::assets::server::AssetServer;
use crate::assets::handle::Handle;

enum AudioCommand {
    PlayOnce {
        clip: AudioClip,
        bus: AudioBusName,
        volume: f32,
        pitch: f32,
    },
    PlayLooped {
        clip: AudioClip,
        bus: AudioBusName,
        volume: f32,
    },
    SetBusVolume {
        bus: AudioBusName,
        volume: f32,
    },
}

/// The central audio playback system.
pub struct AudioManager {
    sender: Sender<AudioCommand>,
    /// Global buses for volume routing.
    pub buses: HashMap<AudioBusName, AudioBus>,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    /// Initialize the audio subsystem.
    pub fn new() -> Self {
        let mut buses = HashMap::new();
        buses.insert(AudioBusName::Master, AudioBus::new(AudioBusName::Master));
        buses.insert(AudioBusName::Music, AudioBus::new(AudioBusName::Music));
        buses.insert(AudioBusName::Sfx, AudioBus::new(AudioBusName::Sfx));
        buses.insert(AudioBusName::UI, AudioBus::new(AudioBusName::UI));

        let (sender, receiver) = mpsc::channel::<AudioCommand>();

        thread::Builder::new()
            .name("veltrix_audio_thread".to_string())
            .spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default()
                    .unwrap_or_else(|e| panic!("Failed to initialize audio stream: {:?}", e));

                let mut thread_buses = HashMap::new();
                thread_buses.insert(AudioBusName::Master, AudioBus::new(AudioBusName::Master));
                thread_buses.insert(AudioBusName::Music, AudioBus::new(AudioBusName::Music));
                thread_buses.insert(AudioBusName::Sfx, AudioBus::new(AudioBusName::Sfx));
                thread_buses.insert(AudioBusName::UI, AudioBus::new(AudioBusName::UI));

                let mut looped_sinks = Vec::new();

                for cmd in receiver {
                    match cmd {
                        AudioCommand::PlayOnce { clip, bus, volume, pitch } => {
                            if let Ok(sink) = Sink::try_new(&stream_handle) {
                                let bus_vol = thread_buses.get(&bus).map(|b| b.volume).unwrap_or(1.0);
                                let master_vol = thread_buses.get(&AudioBusName::Master).map(|b| b.volume).unwrap_or(1.0);
                                
                                sink.set_volume(volume * bus_vol * master_vol);
                                sink.set_speed(pitch);
                                sink.append(clip.decoder());
                                sink.detach();
                            }
                        }
                        AudioCommand::PlayLooped { clip, bus, volume } => {
                            if let Ok(sink) = Sink::try_new(&stream_handle) {
                                let bus_vol = thread_buses.get(&bus).map(|b| b.volume).unwrap_or(1.0);
                                let master_vol = thread_buses.get(&AudioBusName::Master).map(|b| b.volume).unwrap_or(1.0);
                                
                                sink.set_volume(volume * bus_vol * master_vol);
                                sink.append(clip.decoder().repeat_infinite());
                                looped_sinks.push(sink);
                            }
                        }
                        AudioCommand::SetBusVolume { bus, volume } => {
                            if let Some(b) = thread_buses.get_mut(&bus) {
                                b.volume = volume;
                            }
                        }
                    }
                    looped_sinks.retain(|s| !s.empty());
                }
            })
            .expect("Failed to spawn audio background thread");

        Self {
            sender,
            buses,
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
            let _ = self.sender.send(AudioCommand::PlayOnce {
                clip,
                bus: bus_name,
                volume,
                pitch,
            });
        }
    }

    /// Play an audio clip looping on a specific bus.
    pub fn play_looped(
        &mut self,
        assets: &AssetServer,
        clip_handle: &Handle<AudioClip>,
        bus_name: AudioBusName,
        volume: f32,
    ) {
        if let Some(clip) = assets.with_asset(clip_handle, |c| c.clone()) {
            let _ = self.sender.send(AudioCommand::PlayLooped {
                clip,
                bus: bus_name,
                volume,
            });
        }
    }

    pub fn set_bus_volume(&mut self, bus_name: AudioBusName, volume: f32) {
        if let Some(bus) = self.buses.get_mut(&bus_name) {
            bus.volume = volume;
            let _ = self.sender.send(AudioCommand::SetBusVolume { bus: bus_name, volume });
        }
    }
}
