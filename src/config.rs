use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{audio::source::AudioSource, error::Error};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub focus_end_sound: AudioSource,
    #[serde(default)]
    pub rest_end_sound: AudioSource,
}

impl Config {
    pub fn from_default_json() -> Self {
        Self::from_json(Self::file()).unwrap_or_default()
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<Self, Error> {
        let f = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(f)?)
    }

    pub fn to_default_json(&self) -> Result<(), Error> {
        self.to_json(Self::file())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let f = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }

    pub fn dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| ".".into())
            .join("termodoro")
    }

    pub fn file() -> PathBuf {
        Self::dir().join("config.json")
    }
}
