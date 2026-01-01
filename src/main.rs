pub mod core;
pub mod fissure;
pub mod manifests;
pub mod model;
pub mod region;
pub mod world_state;

use std::error::Error;

use reqwest::blocking::get;

use crate::{
    manifests::{ExportRegions, Exports},
    world_state::WorldStateUnmapped,
};

type BoxDynError = Box<dyn Error>;

fn get_export() -> Result<Exports, BoxDynError> {
    let file = get("https://origin.warframe.com/PublicExport/index_en.txt.lzma")?
        .bytes()?
        .to_vec();

    let mut buffer: Vec<u8> = Vec::new();

    lzma_rs::lzma_decompress(&mut file.as_slice(), &mut buffer).unwrap();

    let data = String::from_utf8(buffer)?;

    let export: manifests::PublicExportIndex = data.parse()?;

    Ok(Exports {
        export_regions: get(format!(
            "http://content.warframe.com/PublicExport/Manifest/{}",
            export.regions
        ))?
        .json::<ExportRegions>()?,
    })
}

fn main() -> Result<(), BoxDynError> {
    let exports = get_export()?;

    let fissures = get("https://api.warframe.com/cdn/worldState.php")?
        .json::<WorldStateUnmapped>()?
        .map(exports);

    println!("{fissures:#?}");

    Ok(())
}
