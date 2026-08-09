use gpui::*;
use gpui_component::{ThemeConfig, ThemeConfigColors};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Theme {
    pub background: Rgba,
    pub background_weak: Rgba,
    pub surface: Rgba,
    pub surface_strong: Rgba,
    pub border: Rgba,
    pub text_disabled: Rgba,
    pub text_muted: Rgba,
    pub text: Rgba,
    pub text_strong: Rgba,
    pub error: Rgba,
    pub warning: Rgba,
    pub info: Rgba,
    pub success: Rgba,
    pub accent: Rgba,
    pub accent_alt: Rgba,
    pub accent_muted: Rgba,
    pub special: Rgba,
}

impl Global for Theme {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ThemeType {
    #[default]
    Dark,
    Light,

    Nord,
    NordLight,

    GruvboxDark,
    GruvboxLight,

    TokyoNightDark,
    TokyoNightLight,

    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,

    Custom(Box<Theme>),
}

impl ThemeType {
    pub fn get_theme(&self) -> Theme {
        match self {
            Self::Dark => Theme {
                background: rgba(0x18_18_18_FF),
                background_weak: rgba(0x28_28_28_FF),
                surface: rgba(0x38_38_38_FF),
                surface_strong: rgba(0x58_58_58_FF),
                border: rgba(0x58_58_58_FF),
                text_disabled: rgba(0xB8_B8_B8_FF),
                text_muted: rgba(0xB8_B8_B8_FF),
                text: rgba(0xD8_D8_D8_FF),
                text_strong: rgba(0xE8_E8_E8_FF),
                error: rgba(0xAB_46_42_FF),
                warning: rgba(0xDC_96_56_FF),
                info: rgba(0xF7_CA_88_FF),
                success: rgba(0xA1_B5_6C_FF),
                accent: rgba(0x7C_AF_C2_FF),
                accent_alt: rgba(0x86_C1_B9_FF),
                accent_muted: rgba(0xBA_8B_AF_FF),
                special: rgba(0xA1_69_46_FF),
            },
            Self::Light => Theme {
                background: rgba(0xF8_F8_F8_FF),
                background_weak: rgba(0xE8_E8_E8_FF),
                surface: rgba(0xD8_D8_D8_FF),
                surface_strong: rgba(0xB8_B8_B8_FF),
                border: rgba(0xB8_B8_B8_FF),
                text_disabled: rgba(0x58_58_58_FF),
                text_muted: rgba(0x58_58_58_FF),
                text: rgba(0x38_38_38_FF),
                text_strong: rgba(0x28_28_28_FF),
                error: rgba(0xAB_46_42_FF),
                warning: rgba(0xDC_96_56_FF),
                info: rgba(0xF7_CA_88_FF),
                success: rgba(0xA1_B5_6C_FF),
                accent: rgba(0x7C_AF_C2_FF),
                accent_alt: rgba(0x86_C1_B9_FF),
                accent_muted: rgba(0xBA_8B_AF_FF),
                special: rgba(0xA1_69_46_FF),
            },

            Self::Nord => Theme {
                background: rgba(0x2E_34_40_FF),
                background_weak: rgba(0x3B_42_52_FF),
                surface: rgba(0x43_4C_5E_FF),
                surface_strong: rgba(0x4C_56_6A_FF),
                border: rgba(0x4C_56_6A_FF),
                text_disabled: rgba(0x4C_56_6A_FF),
                text_muted: rgba(0xD8_DE_E9_FF),
                text: rgba(0xE5_E9_F0_FF),
                text_strong: rgba(0xEC_EF_F4_FF),
                error: rgba(0xBF_61_6A_FF),
                warning: rgba(0xD0_87_70_FF),
                info: rgba(0xEB_CB_8B_FF),
                success: rgba(0xA3_BE_8C_FF),
                accent: rgba(0x81_A1_C1_FF),
                accent_alt: rgba(0x88_C0_D0_FF),
                accent_muted: rgba(0xB4_8E_AD_FF),
                special: rgba(0x5E_81_AC_FF),
            },
            Self::NordLight => Theme {
                background: rgba(0xE5_E9_F0_FF),
                background_weak: rgba(0xC2_D0_E7_FF),
                surface: rgba(0xB8_C5_DB_FF),
                surface_strong: rgba(0xAE_BA_CF_FF),
                border: rgba(0xAE_BA_CF_FF),
                text_disabled: rgba(0x60_72_8C_FF),
                text_muted: rgba(0x60_72_8C_FF),
                text: rgba(0x2E_34_40_FF),
                text_strong: rgba(0x3B_42_52_FF),
                error: rgba(0x99_32_4B_FF),
                warning: rgba(0xAC_44_26_FF),
                info: rgba(0x9A_75_00_FF),
                success: rgba(0x4F_89_4C_FF),
                accent: rgba(0x3B_6E_A8_FF),
                accent_alt: rgba(0x39_8E_AC_FF),
                accent_muted: rgba(0x97_36_5B_FF),
                special: rgba(0x52_72_AF_FF),
            },

            Self::GruvboxDark => Theme {
                background: rgba(0x28_28_28_FF),
                background_weak: rgba(0x3C_38_36_FF),
                surface: rgba(0x50_49_45_FF),
                surface_strong: rgba(0x66_5C_54_FF),
                border: rgba(0x66_5C_54_FF),
                text_disabled: rgba(0x92_83_74_FF),
                text_muted: rgba(0x92_83_74_FF),
                text: rgba(0xEB_DB_B2_FF),
                text_strong: rgba(0xFB_F1_C7_FF),
                error: rgba(0xCC_24_1D_FF),
                warning: rgba(0xD6_5D_0E_FF),
                info: rgba(0xD7_99_21_FF),
                success: rgba(0x98_97_1A_FF),
                accent: rgba(0x45_85_88_FF),
                accent_alt: rgba(0x68_9D_6A_FF),
                accent_muted: rgba(0xB1_62_86_FF),
                special: rgba(0x9D_00_06_FF),
            },
            Self::GruvboxLight => Theme {
                background: rgba(0xFB_F1_C7_FF),
                background_weak: rgba(0xEB_DB_B2_FF),
                surface: rgba(0xD5_C4_A1_FF),
                surface_strong: rgba(0xBD_AE_93_FF),
                border: rgba(0xBD_AE_93_FF),
                text_disabled: rgba(0x7C_6F_64_FF),
                text_muted: rgba(0x7C_6F_64_FF),
                text: rgba(0x3C_38_36_FF),
                text_strong: rgba(0x28_28_28_FF),
                error: rgba(0xCC_24_1D_FF),
                warning: rgba(0xD6_5D_0E_FF),
                info: rgba(0xD7_99_21_FF),
                success: rgba(0x98_97_1A_FF),
                accent: rgba(0x45_85_88_FF),
                accent_alt: rgba(0x68_9D_6A_FF),
                accent_muted: rgba(0xB1_62_86_FF),
                special: rgba(0x9D_00_06_FF),
            },

            Self::TokyoNightDark => Theme {
                background: rgba(0x1A_1B_26_FF),
                background_weak: rgba(0x16_16_1E_FF),
                surface: rgba(0x2F_35_49_FF),
                surface_strong: rgba(0x44_4B_6A_FF),
                border: rgba(0x44_4B_6A_FF),
                text_disabled: rgba(0x78_7C_99_FF),
                text_muted: rgba(0x78_7C_99_FF),
                text: rgba(0xA9_B1_D6_FF),
                text_strong: rgba(0xCB_CC_D1_FF),
                error: rgba(0xC0_CA_F5_FF),
                warning: rgba(0xA9_B1_D6_FF),
                info: rgba(0x0D_B9_D7_FF),
                success: rgba(0x9E_CE_6A_FF),
                accent: rgba(0x2A_C3_DE_FF),
                accent_alt: rgba(0xB4_F9_F8_FF),
                accent_muted: rgba(0xBB_9A_F7_FF),
                special: rgba(0xF7_76_8E_FF),
            },
            Self::TokyoNightLight => Theme {
                background: rgba(0xD5_D6_DB_FF),
                background_weak: rgba(0xCB_CC_D1_FF),
                surface: rgba(0xDF_E0_E5_FF),
                surface_strong: rgba(0x96_99_A3_FF),
                border: rgba(0x96_99_A3_FF),
                text_disabled: rgba(0x4C_50_5E_FF),
                text_muted: rgba(0x4C_50_5E_FF),
                text: rgba(0x34_3B_59_FF),
                text_strong: rgba(0x1A_1B_26_FF),
                error: rgba(0x34_3B_58_FF),
                warning: rgba(0x96_50_27_FF),
                info: rgba(0x16_67_75_FF),
                success: rgba(0x48_5E_30_FF),
                accent: rgba(0x34_54_8A_FF),
                accent_alt: rgba(0x3E_69_68_FF),
                accent_muted: rgba(0x5A_4A_78_FF),
                special: rgba(0x8C_43_51_FF),
            },

            Self::CatppuccinFrappe => Theme {
                background: rgba(0x30_34_46_FF),
                background_weak: rgba(0x41_45_59_FF),
                surface: rgba(0x51_57_6D_FF),
                surface_strong: rgba(0x73_79_94_FF),
                border: rgba(0x73_79_94_FF),
                text_disabled: rgba(0xA5_AD_CE_FF),
                text_muted: rgba(0xA5_AD_CE_FF),
                text: rgba(0xC6_D0_F5_FF),
                text_strong: rgba(0xF2_D5_CF_FF),
                error: rgba(0xE7_82_84_FF),
                warning: rgba(0xEF_9F_76_FF),
                info: rgba(0xE5_C8_90_FF),
                success: rgba(0xA6_D1_89_FF),
                accent: rgba(0x8C_AA_EE_FF),
                accent_alt: rgba(0x81_C8_BE_FF),
                accent_muted: rgba(0xCA_9E_E6_FF),
                special: rgba(0xEE_BE_BE_FF),
            },
            Self::CatppuccinLatte => Theme {
                background: rgba(0xEF_F1_F5_FF),
                background_weak: rgba(0xCC_D0_DA_FF),
                surface: rgba(0xBC_C0_CC_FF),
                surface_strong: rgba(0x9C_A0_B0_FF),
                border: rgba(0x9C_A0_B0_FF),
                text_disabled: rgba(0x6C_6F_85_FF),
                text_muted: rgba(0x6C_6F_85_FF),
                text: rgba(0x4C_4F_69_FF),
                text_strong: rgba(0xDC_8A_78_FF),
                error: rgba(0xD2_0F_39_FF),
                warning: rgba(0xFE_64_0B_FF),
                info: rgba(0xDF_8E_1D_FF),
                success: rgba(0x40_A0_2B_FF),
                accent: rgba(0x1E_66_F5_FF),
                accent_alt: rgba(0x17_92_99_FF),
                accent_muted: rgba(0x88_39_EF_FF),
                special: rgba(0xDD_78_78_FF),
            },
            Self::CatppuccinMacchiato => Theme {
                background: rgba(0x24_27_3A_FF),
                background_weak: rgba(0x36_3A_4F_FF),
                surface: rgba(0x49_4D_64_FF),
                surface_strong: rgba(0x6E_73_8D_FF),
                border: rgba(0x6E_73_8D_FF),
                text_disabled: rgba(0xA5_AD_CB_FF),
                text_muted: rgba(0xA5_AD_CB_FF),
                text: rgba(0xCA_D3_F5_FF),
                text_strong: rgba(0xF4_DB_D6_FF),
                error: rgba(0xED_87_96_FF),
                warning: rgba(0xF5_A9_7F_FF),
                info: rgba(0xEE_D4_9F_FF),
                success: rgba(0xA6_DA_95_FF),
                accent: rgba(0x8A_AD_F4_FF),
                accent_alt: rgba(0x8B_D5_CA_FF),
                accent_muted: rgba(0xC6_A0_F6_FF),
                special: rgba(0xF0_C6_C6_FF),
            },
            Self::CatppuccinMocha => Theme {
                background: rgba(0x1E_1E_2E_FF),
                background_weak: rgba(0x31_32_44_FF),
                surface: rgba(0x45_47_5A_FF),
                surface_strong: rgba(0x6C_70_86_FF),
                border: rgba(0x6C_70_86_FF),
                text_disabled: rgba(0xA6_AD_C8_FF),
                text_muted: rgba(0xA6_AD_C8_FF),
                text: rgba(0xCD_D6_F4_FF),
                text_strong: rgba(0xF5_E0_DC_FF),
                error: rgba(0xF3_8B_A8_FF),
                warning: rgba(0xFA_B3_87_FF),
                info: rgba(0xF9_E2_AF_FF),
                success: rgba(0xA6_E3_A1_FF),
                accent: rgba(0x89_B4_FA_FF),
                accent_alt: rgba(0x94_E2_D5_FF),
                accent_muted: rgba(0xCB_A6_F7_FF),
                special: rgba(0xF2_CD_CD_FF),
            },

            Self::Custom(theme) => **theme,
        }
    }
}

impl From<Theme> for ThemeConfig {
    fn from(value: Theme) -> Self {
        let to_hex = |color: Rgba| -> SharedString {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8
            )
            .into()
        };

        let mut colors = ThemeConfigColors::default();

        colors.background = Some(to_hex(value.background));
        colors.foreground = Some(to_hex(value.text));
        colors.border = Some(to_hex(value.border));
        colors.ring = Some(to_hex(value.accent));
        colors.caret = Some(to_hex(value.accent));

        colors.accent = Some(to_hex(value.background_weak));
        colors.accent_foreground = Some(to_hex(value.text_strong));
        colors.selection = Some(to_hex(value.surface));

        colors.input = Some(to_hex(value.border));
        colors.muted = Some(to_hex(value.background_weak));
        colors.muted_foreground = Some(to_hex(value.text_disabled));

        colors.primary = Some(to_hex(value.accent));
        colors.primary_hover = Some(to_hex(value.accent_alt));
        colors.primary_active = Some(to_hex(value.special));
        colors.primary_foreground = Some(to_hex(value.background));

        colors.secondary = Some(to_hex(value.surface));
        colors.secondary_hover = Some(to_hex(value.surface_strong));
        colors.secondary_active = Some(to_hex(value.background_weak));
        colors.secondary_foreground = Some(to_hex(value.text_strong));

        colors.accordion = Some(to_hex(value.background));
        colors.accordion_hover = Some(to_hex(value.background_weak));
        colors.group_box = Some(to_hex(value.background_weak));
        colors.group_box_foreground = Some(to_hex(value.text));
        colors.group_box_title_foreground = Some(to_hex(value.text_strong));

        colors.popover = Some(to_hex(value.background_weak));
        colors.popover_foreground = Some(to_hex(value.text));
        colors.overlay = Some(to_hex(value.background));

        colors.list = Some(to_hex(value.background));
        colors.list_even = Some(to_hex(value.background_weak));
        colors.list_head = Some(to_hex(value.surface));
        colors.list_hover = Some(to_hex(value.surface));
        colors.list_active = Some(to_hex(value.surface_strong));
        colors.list_active_border = Some(to_hex(value.accent));

        colors.table = Some(to_hex(value.background));
        colors.table_even = Some(to_hex(value.background_weak));
        colors.table_head = Some(to_hex(value.surface));
        colors.table_head_foreground = Some(to_hex(value.text_strong));
        colors.table_hover = Some(to_hex(value.surface));
        colors.table_active = Some(to_hex(value.surface_strong));
        colors.table_active_border = Some(to_hex(value.accent));
        colors.table_row_border = Some(to_hex(value.border));

        colors.sidebar = Some(to_hex(value.background_weak));
        colors.sidebar_border = Some(to_hex(value.border));
        colors.sidebar_foreground = Some(to_hex(value.text_muted));
        colors.sidebar_accent = Some(to_hex(value.surface));
        colors.sidebar_accent_foreground = Some(to_hex(value.text_strong));
        colors.sidebar_primary = Some(to_hex(value.accent));
        colors.sidebar_primary_foreground = Some(to_hex(value.background));

        colors.tab = Some(to_hex(value.background_weak));
        colors.tab_foreground = Some(to_hex(value.text_disabled));
        colors.tab_active = Some(to_hex(value.background));
        colors.tab_active_foreground = Some(to_hex(value.text_strong));
        colors.tab_bar = Some(to_hex(value.surface));
        colors.tab_bar_segmented = Some(to_hex(value.surface_strong));
        colors.title_bar = Some(to_hex(value.background_weak));
        colors.title_bar_border = Some(to_hex(value.border));
        colors.window_border = Some(to_hex(value.border));
        colors.tiles = Some(to_hex(value.background));

        colors.scrollbar = Some(to_hex(value.background));
        colors.scrollbar_thumb = Some(to_hex(value.surface));
        colors.scrollbar_thumb_hover = Some(to_hex(value.surface_strong));
        colors.slider_bar = Some(to_hex(value.surface));
        colors.slider_thumb = Some(to_hex(value.accent));
        colors.progress_bar = Some(to_hex(value.accent));

        colors.danger = Some(to_hex(value.error));
        colors.danger_hover = Some(to_hex(value.error));
        colors.danger_active = Some(to_hex(value.error));
        colors.danger_foreground = Some(to_hex(value.text_strong));

        colors.warning = Some(to_hex(value.warning));
        colors.warning_hover = Some(to_hex(value.warning));
        colors.warning_active = Some(to_hex(value.warning));
        colors.warning_foreground = Some(to_hex(value.text_strong));

        colors.info = Some(to_hex(value.info));
        colors.info_hover = Some(to_hex(value.info));
        colors.info_active = Some(to_hex(value.info));
        colors.info_foreground = Some(to_hex(value.text_strong));

        colors.success = Some(to_hex(value.success));
        colors.success_hover = Some(to_hex(value.success));
        colors.success_active = Some(to_hex(value.success));
        colors.success_foreground = Some(to_hex(value.text_strong));

        colors.skeleton = Some(to_hex(value.surface));
        colors.switch = Some(to_hex(value.surface));
        colors.switch_thumb = Some(to_hex(value.text_strong));
        colors.drag_border = Some(to_hex(value.accent_alt));
        colors.drop_target = Some(to_hex(value.surface));
        colors.link = Some(to_hex(value.accent));
        colors.link_hover = Some(to_hex(value.accent_alt));
        colors.link_active = Some(to_hex(value.special));

        colors.chart_1 = Some(to_hex(value.accent));
        colors.chart_2 = Some(to_hex(value.accent_alt));
        colors.chart_3 = Some(to_hex(value.special));
        colors.chart_4 = Some(to_hex(value.accent_muted));
        colors.chart_5 = Some(to_hex(value.success));
        colors.bullish = Some(to_hex(value.success));
        colors.bearish = Some(to_hex(value.error));

        Self {
            colors,
            ..Default::default()
        }
    }
}
