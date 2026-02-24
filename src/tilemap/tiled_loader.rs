//! Parses Tiled `.tmx` map files.

use anyhow::{Context, Result};
use std::path::Path;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use super::{tilemap::Tilemap, tileset::Tileset};

/// Stub implementation for loading Tiled (`.tmx`) format maps.
pub struct TiledLoader;

impl TiledLoader {
    /// Load a `.tmx` file from disk.
    ///
    /// This is a basic demonstration of XML parsing. In a production engine, 
    /// you should use the `tiled` crate for comprehensive support.
    pub fn load_tmx(path: impl AsRef<Path>) -> Result<(Vec<Tilemap>, Vec<Tileset>)> {
        let path = path.as_ref();
        let src = std::fs::read_to_string(path)
            .with_context(|| format!("Reading TMX file {:?}", path))?;

        let mut reader = Reader::from_str(&src);
        // reader.config_mut().trim_text(true);

        let mut tilemaps = Vec::new();
        let mut tilesets = Vec::new();
        
        let mut buf = Vec::new();
        
        // This is a minimal stub simulating TMX parsing. A real parser would
        // traverse all <layer> and <tileset> elements correctly.
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"map" => {
                            // Read map dimensions and tile size
                        }
                        b"tileset" => {
                            // Read firstgid, name, tilewidth, tileheight
                            // Inside this, parse <image source="...">
                            tilesets.push(Tileset {
                                name: "dummy".to_string(),
                                texture: None,
                                image_width: 256,
                                image_height: 256,
                                tile_width: 16,
                                tile_height: 16,
                                first_gid: 1,
                                tile_count: 256,
                            });
                        }
                        b"layer" => {
                            // Parse <data encoding="csv"> ... </data>
                            tilemaps.push(Tilemap::new(10, 10, 16, 16));
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow::anyhow!("XML Parsing error: {}", e)),
                _ => (),
            }
            buf.clear();
        }

        Ok((tilemaps, tilesets))
    }
}
