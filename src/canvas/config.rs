use inquire_derive::Selectable;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Deserialize, Serialize, Default)]
pub struct CanvasConfig {
    pub base_url: Option<String>,
    pub access_token: Option<String>
}

#[derive(Debug, Clone, Copy, Selectable, Display)]
pub enum CanvasConfigOption {
    #[strum(to_string = "Base URL")]
    BaseUrl,
    #[strum(to_string = "Access Token")]
    AccessToken,
    
}

