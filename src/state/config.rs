use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
};
use color_eyre::owo_colors::OwoColorize as _;
use iced::Theme as BuiltinIcedTheme;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config
{
    pub general: General,
    pub theme: Theme,
}

impl Config
{
    pub fn read_config_file() -> Result<Self, Error>
    {
        let default_toml = match toml::to_string_pretty(&Self::default())
        {
            Ok(toml) => toml,
            Err(err) =>
            {
                panic!(
                    "{}",
                    error!(Error::Parse, "default config", err)
                        .get_message()
                        .red()
                )
            }
        };

        let config_path = AppState::get_config_dir("config.toml", Some(default_toml))?;

        match &fs::read_to_string(&config_path)
        {
            Ok(string) => match toml::from_str(string)
            {
                Ok(config) => Ok(config),
                Err(err) => Err(error!(Error::Parse, "config", err)),
            },
            Err(err) => Err(error!(Error::Parse, "config", err)),
        }
    }

    pub fn is_valid(&self) -> Result<(), String>
    {
        let projects_dir = &self.general.projects_dir;
        if projects_dir.is_empty()
            || !PathBuf::from(&projects_dir).is_dir()
            || !projects_dir.ends_with('/')
        {
            return Err(String::from(
                "Error: Failed to run projman please make sure projects_dir \
                in .config/projman/config.toml is a valid directory \
                (Dont forget the trailing slash!!)",
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct General
{
    pub projects_dir: String,
    pub delete_project_folder: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Theme
{
    pub theme: IcedTheme,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub enum IcedTheme
{
    Light,
    #[default]
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl IcedTheme
{
    pub fn convert_to_iced_theme(&self) -> BuiltinIcedTheme
    {
        match self
        {
            Self::Dark => BuiltinIcedTheme::Dark,
            Self::Light => BuiltinIcedTheme::Light,
            Self::Dracula => BuiltinIcedTheme::Dracula,
            Self::Nord => BuiltinIcedTheme::Nord,
            Self::SolarizedLight => BuiltinIcedTheme::SolarizedLight,
            Self::SolarizedDark => BuiltinIcedTheme::SolarizedDark,
            Self::GruvboxLight => BuiltinIcedTheme::GruvboxLight,
            Self::GruvboxDark => BuiltinIcedTheme::GruvboxDark,
            Self::CatppuccinLatte => BuiltinIcedTheme::CatppuccinLatte,
            Self::CatppuccinFrappe => BuiltinIcedTheme::CatppuccinFrappe,
            Self::CatppuccinMacchiato => BuiltinIcedTheme::CatppuccinMacchiato,
            Self::CatppuccinMocha => BuiltinIcedTheme::CatppuccinMocha,
            Self::TokyoNight => BuiltinIcedTheme::TokyoNight,
            Self::TokyoNightStorm => BuiltinIcedTheme::TokyoNightStorm,
            Self::TokyoNightLight => BuiltinIcedTheme::TokyoNightLight,
            Self::KanagawaWave => BuiltinIcedTheme::KanagawaWave,
            Self::KanagawaDragon => BuiltinIcedTheme::KanagawaDragon,
            Self::KanagawaLotus => BuiltinIcedTheme::KanagawaLotus,
            Self::Moonfly => BuiltinIcedTheme::Moonfly,
            Self::Nightfly => BuiltinIcedTheme::Nightfly,
            Self::Oxocarbon => BuiltinIcedTheme::Oxocarbon,
            Self::Ferra => BuiltinIcedTheme::Ferra,
        }
    }
}
