// SPDX-FileCopyrightText: Copyright (c) 2026 Juhan280
// SPDX-License-Identifier: MIT

use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use ron::ser::PrettyConfig;

pub fn read_ron_from_file<T: serde::de::DeserializeOwned>(config_path: &Path) -> Result<T, ()> {
    log::info!("Reading RON file from: {}", config_path.display());
    let file = File::open(config_path).map_err(|e| {
        log::error!(
            "Failed to read RON file from {}:\n  {e}",
            config_path.display(),
        )
    })?;

    log::debug!("Deserializing RON data from {}", config_path.display());
    ron::de::from_reader(BufReader::new(file))
        .map_err(|e| log::error!("Failed to parse RON file format (corrupt RON layout):\n  {e}"))
}

pub fn save_ron_to_file<T: ?Sized + serde::Serialize>(path: &Path, data: &T) -> Result<(), ()> {
    log::debug!("Serializing data to RON for {}", path.display());
    let str = ron::ser::to_string_pretty(&data, PrettyConfig::new())
        .map_err(|e| log::error!("Failed to serialize configuration back to RON: {}", e))?;

    let temp_path = path.with_extension("tmp");

    log::debug!("Writing data to temp_file at: {}", temp_path.display());
    fs::write(&temp_path, str).map_err(|e| {
        log::error!(
            "Failed to write temporary configuration file at {}: {e}",
            temp_path.display(),
        )
    })?;

    log::info!(
        "Attempting to atomically save modifications to {}",
        path.display()
    );
    fs::rename(&temp_path, path).map_err(|e| {
        log::error!(
            "Failed to atomically save modifications to disk at {}: {e}",
            path.display(),
        )
    })?;

    Ok(())
}
