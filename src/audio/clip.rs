//! The AudioClip asset type.

use std::io::Cursor;
use rodio::{Decoder, Source};

use crate::assets::server::Asset;

/// Represents a loaded block of audio data (WAV, OGG, or MP3) ready for playback.
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub(crate) data: Vec<u8>,
}

impl Asset for AudioClip {
    fn load(bytes: &[u8], _ext: &str) -> anyhow::Result<Self> {
        // Just verify it decodes
        let cursor = Cursor::new(bytes.to_vec());
        let _decoder = Decoder::new(cursor)?;
        
        Ok(Self {
            data: bytes.to_vec(),
        })
    }
}

impl AudioClip {
    /// Decode the clip into a rodio Source for playback.
    ///
    /// This is internally called by the AudioManager each time the clip is played.
    pub fn decoder(&self) -> Decoder<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(self.data.clone());
        Decoder::new(cursor).expect("Failed to decode previously validated audio clip")
    }
}
