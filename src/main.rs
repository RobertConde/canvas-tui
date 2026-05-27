use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};
use strum::Display;

use canvas_tui::{
    canvas::config::{CanvasConfig, CanvasConfigOption},
    tui::config::{TuiConfig, TuiConfigOption, default_config_path},
};
use inquire::{self, Text, validator::Validation};
use inquire_derive::Selectable;

fn tui_config() -> TuiConfig {
    let default_path = CONFIG_PATH.lock().unwrap().to_string_lossy().into_owned();

    let config_path = Text::new("Config path:")
        .with_default(default_path.as_str())
        // .with_autocomplete(FilePathCompleter) // tab-completes paths
        .with_validator(|input: &str| {
            if Path::new(input).exists() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Path does not exist".into()))
            }
        })
        .prompt()
        .unwrap();

    TuiConfig::load(Path::new(&config_path)).unwrap()
}

#[derive(Debug, Copy, Clone, Display, Selectable)]
enum MenuOption {
    #[strum(to_string = "Configure")]
    Configure,
}

fn main_menu() -> () {
    match MenuOption::select("What shall we do?")
        .prompt_skippable()
        .unwrap()
    {
        Some(MenuOption::Configure) => configure(),
        None => (),
    };
}

fn configure() -> () {
    match TuiConfigOption::select("What would you like to configure?")
        .prompt_skippable()
        .unwrap()
    {
        Some(TuiConfigOption::CanvasConfig) => configure_canvas(),
        None => (),
    }
}

fn configure_canvas() {
    let mut config = CONFIG.lock().unwrap();
    let canvas = config
        .canvas_config
        .get_or_insert_with(CanvasConfig::default);

    loop {
        let configure_choice = CanvasConfigOption::select("Edit which Canvas configuration?")
            .prompt_skippable()
            .unwrap();

        let (field, name) = match configure_choice {
            Some(CanvasConfigOption::BaseUrl) => (&mut canvas.base_url, "Base URL"),
            Some(CanvasConfigOption::AccessToken) => (&mut canvas.access_token, "Access Token"),
            None => break,
        };

        let current = field.as_deref().unwrap_or("").to_string();

        let new_value = Text::new(&format!("{}:", name))
            .with_default(&current) // borrows current, not field
            .prompt()
            .unwrap();

        *field = Some(new_value);
    }
}

static CONFIG_PATH: LazyLock<Mutex<PathBuf>> =
    LazyLock::new(|| Mutex::new(default_config_path().clone()));
static CONFIG: LazyLock<Mutex<TuiConfig>> = LazyLock::new(|| Mutex::new(tui_config()));

fn main() {
    LazyLock::force(&CONFIG);

    main_menu();

    // Write config
    let config = CONFIG.lock().unwrap();
    let path = CONFIG_PATH.lock().unwrap();
    config.write_config(&path).unwrap();

    println!("\nMade with ❤️ by Rob. Bye now! :)")
}
