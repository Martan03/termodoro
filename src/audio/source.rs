use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{audio::player::Player, error::Error};

#[cfg(feature = "default-sounds")]
const DEFAULT_FOCUS_END: &[u8] = include_bytes!("../../assets/focus.wav");
#[cfg(feature = "default-sounds")]
const DEFAULT_REST_END: &[u8] = include_bytes!("../../assets/rest.wav");

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub enum AudioSource {
    #[default]
    Default,
    Custom(PathBuf),
    None,
}

impl AudioSource {
    pub fn play(&self, player: &mut Player, rest: bool) -> Result<(), Error> {
        match self {
            Self::Default => Self::play_embed(player, rest),
            Self::Custom(path) if path.exists() => player.play(path),
            _ => Ok(()),
        }
    }

    fn play_embed(player: &mut Player, rest: bool) -> Result<(), Error> {
        #[cfg(feature = "default-sounds")]
        {
            match rest {
                true => player.play_embed(DEFAULT_FOCUS_END),
                false => player.play_embed(DEFAULT_REST_END),
            }
        }
        #[cfg(not(feature = "default-sounds"))]
        Ok(())
    }
}
