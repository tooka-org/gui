#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::app::Tooka;
use iced::advanced::graphics::image::image_rs;

pub mod app;
pub mod pages;
pub mod theme;

static LOGO: &[u8] = include_bytes!("../assets/logo.png").as_slice();
static ONLINE: &[u8] = include_bytes!("../assets/icons/online.svg").as_slice();
static CHEVRON_DOWN: &[u8] = include_bytes!("../assets/icons/chevron-down.svg").as_slice();
static CHEVRON_LEFT: &[u8] = include_bytes!("../assets/icons/chevron-left.svg").as_slice();
static ARROW_BADGE_DOWN: &[u8] = include_bytes!("../assets/icons/arrow-badge-down.svg").as_slice();
static ARROW_BADGE_UP: &[u8] = include_bytes!("../assets/icons/arrow-badge-up.svg").as_slice();
static CHECK: &[u8] = include_bytes!("../assets/icons/check.svg").as_slice();
static CLIPBOARD_DATA: &[u8] = include_bytes!("../assets/icons/clipboard-data.svg").as_slice();
static CPU: &[u8] = include_bytes!("../assets/icons/cpu.svg").as_slice();
static DATABASE_IMPORT: &[u8] = include_bytes!("../assets/icons/database-import.svg").as_slice();
static USB: &[u8] = include_bytes!("../assets/icons/usb.svg").as_slice();
static HOURGLASS_HIGH: &[u8] = include_bytes!("../assets/icons/hourglass-high.svg").as_slice();
static DOWNLOAD: &[u8] = include_bytes!("../assets/icons/download.svg").as_slice();
static EXCLAMATION_CIRCLE: &[u8] =
    include_bytes!("../assets/icons/exclamation-circle.svg").as_slice();
static FILE: &[u8] = include_bytes!("../assets/icons/file.svg").as_slice();
static FILE_DESCRIPTION: &[u8] = include_bytes!("../assets/icons/file-description.svg").as_slice();
static FILE_DOWNLOAD: &[u8] = include_bytes!("../assets/icons/file-download.svg").as_slice();
static FOLDER: &[u8] = include_bytes!("../assets/icons/folder.svg").as_slice();
static GIT_COMMIT: &[u8] = include_bytes!("../assets/icons/git-commit.svg").as_slice();
static GLOBE: &[u8] = include_bytes!("../assets/icons/globe.svg").as_slice();
static HOME: &[u8] = include_bytes!("../assets/icons/home.svg").as_slice();
static LICENSE: &[u8] = include_bytes!("../assets/icons/license.svg").as_slice();
static REFRESH: &[u8] = include_bytes!("../assets/icons/refresh.svg").as_slice();
static SETTINGS: &[u8] = include_bytes!("../assets/icons/settings.svg").as_slice();
static USER_CODE: &[u8] = include_bytes!("../assets/icons/user-code.svg").as_slice();
static THEME_TOGGLE: &[u8] = include_bytes!("../assets/icons/theme-toggle.svg").as_slice();
static ZOOM: &[u8] = include_bytes!("../assets/icons/zoom.svg").as_slice();
static _KEY: &[u8] = include_bytes!("../assets/icons/key.svg").as_slice();

fn main() -> Result<(), String> {
    let mut settings = iced::Settings::default();
    let mut window_settings = iced::window::Settings::default();
    settings.id = Some("io.github.tooka-org.tooka".to_owned());
    settings.default_font = iced::Font::default();
    window_settings.icon =
        iced::window::icon::from_file_data(LOGO, Some(image_rs::ImageFormat::Png)).ok();
    window_settings.size = iced::Size::new(400.0, 500.0);

    iced::application("Tooka", Tooka::update, Tooka::view)
        .settings(settings)
        .exit_on_close_request(true)
        .window(window_settings)
        .centered()
        .theme(|_app| iced::Theme::Dark)
        .run()
        .expect("Failed to run application");

    Ok(())
}

//        .subscription(Tooka::subscription)
