use std::{fs::read_to_string, path::PathBuf};

use color_eyre::owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config
{
    pub general: General,
    pub theme: Theme,
}

impl Config
{
    pub fn read_config_file() -> Result<Config, Error>
    {
        let default_toml: String = match toml::to_string_pretty(&Config::default())
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

        let config_path: PathBuf =
            AppState::get_config_dir(String::from("config.toml"), Some(default_toml))?;

        match &read_to_string(&config_path)
        {
            Ok(string) => match toml::from_str(string)
            {
                Ok(config) => return Ok(config),
                Err(err) => return Err(error!(Error::Parse, "config", err)),
            },
            Err(err) => return Err(error!(Error::Parse, "config", err)),
        }
    }

    pub fn is_valid(&self) -> Result<(), String>
    {
        let projects_dir: &String = &self.general.projects_dir;
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
    pub fn convert_to_iced_theme(&self) -> iced::Theme
    {
        match self
        {
            IcedTheme::Dark => iced::Theme::Dark,
            IcedTheme::Light => iced::Theme::Light,
            IcedTheme::Dracula => iced::Theme::Dracula,
            IcedTheme::Nord => iced::Theme::Nord,
            IcedTheme::SolarizedLight => iced::Theme::SolarizedLight,
            IcedTheme::SolarizedDark => iced::Theme::SolarizedDark,
            IcedTheme::GruvboxLight => iced::Theme::GruvboxLight,
            IcedTheme::GruvboxDark => iced::Theme::GruvboxDark,
            IcedTheme::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            IcedTheme::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            IcedTheme::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            IcedTheme::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            IcedTheme::TokyoNight => iced::Theme::TokyoNight,
            IcedTheme::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            IcedTheme::TokyoNightLight => iced::Theme::TokyoNightLight,
            IcedTheme::KanagawaWave => iced::Theme::KanagawaWave,
            IcedTheme::KanagawaDragon => iced::Theme::KanagawaDragon,
            IcedTheme::KanagawaLotus => iced::Theme::KanagawaLotus,
            IcedTheme::Moonfly => iced::Theme::Moonfly,
            IcedTheme::Nightfly => iced::Theme::Nightfly,
            IcedTheme::Oxocarbon => iced::Theme::Oxocarbon,
            IcedTheme::Ferra => iced::Theme::Ferra,
        }
    }
}
