//! Example 1: Hello Window
//! Creates an empty Veltrix engine wrapper and runs the main loop.
//! Shows the clear color and basic window creation.

use anyhow::Result;
use veltrix::prelude::*;

fn main() -> Result<()> {
    // 1. Configure the engine
    let config = Config {
        title: "Veltrix - Hello Window".to_string(),
        width: 800,
        height: 600,
        ..Default::default()
    };

    // 2. Build the engine
    let engine = EngineBuilder::new()
        .with_config(config)
        .build()?;

    // 3. Run the game loop with empty callbacks
    engine.run(
        // on_start
        |_world, _resources| {
            log::info!("Engine started successfully!");
        },
        // on_update
        |_world, _resources, _dt| { true },
        // on_fixed
        |_world, _resources, _fixed_dt| {},
        // on_render
        |_world, _resources, _alpha| {},
    )?;

    Ok(())
}
