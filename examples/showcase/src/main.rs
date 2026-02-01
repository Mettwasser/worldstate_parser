use std::{fs, path::Path};

use worldstate_parser::{default_context_provider::DefaultContextProvider, PathContext, worldstate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let world_state = worldstate::from_str(
        &fs::read_to_string("worldstate.json")?,
        DefaultContextProvider,
        PathContext {
            data_dir: Path::new("data/"),
            drops_dir: Path::new("drops/"),
            assets_dir: Path::new("assets/"),
        },
    )
    .await?;

    fs::write(
        "showcase/worldstate_parsed.json",
        serde_json::to_string_pretty(&world_state)?,
    )?;

    Ok(())
}
