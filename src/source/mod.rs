mod flac_loader;
mod mp3_loader;
mod ogg_loader;
#[cfg(feature = "settings_loader")]
mod settings_loader;
mod wav_loader;

use bevy::reflect::{TypeUuid, TypePath};
use kira::sound::Sound;

pub use flac_loader::FlacLoader;
pub use mp3_loader::Mp3Loader;
pub use ogg_loader::OggLoader;
#[cfg(feature = "settings_loader")]
pub use settings_loader::SettingsLoader;
pub use wav_loader::WavLoader;

/// A source of audio data
#[derive(Debug, Clone, TypeUuid, TypePath)]
#[uuid = "6a9fc4ca-b5b5-94d6-613c-522e2d9fe86d"]
pub struct AudioSource {
    /// The Kira sound making up this `AudioSource`
    pub sound: Sound,
}
