use serde::Deserialize;
use serenity::model::id::{GuildId, RoleId, UserId};
use serenity::prelude::TypeMapKey;
use std::fs::read_to_string;

pub struct StaticConfig;
impl TypeMapKey for StaticConfig {
    type Value = Config;
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub roles: RolesConfig,
    pub links: LinksConfig,
    pub vigil: VigilConfig,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub id: UserId,
    pub guild: GuildId,
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct RolesConfig {
    pub grades: GradesRolesConfig,
    pub platforms: PlatformsRolesConfig,
    pub beta: RoleId,
}

#[derive(Debug, Deserialize)]
pub struct GradesRolesConfig {
    pub freshman: RoleId,
    pub sophomore: RoleId,
    pub junior: RoleId,
    pub senior: RoleId,
    pub graduate: RoleId,
}

#[derive(Debug, Deserialize)]
pub struct PlatformsRolesConfig {
    pub ios: RoleId,
    pub android: RoleId,
}

#[derive(Debug, Deserialize)]
pub struct LinksConfig {
    pub beta: BetaLinksConfig,
}

#[derive(Debug, Deserialize)]
pub struct BetaLinksConfig {
    pub ios: String,
    pub android: String,
}

#[derive(Debug, Deserialize)]
pub struct VigilConfig {
    pub service_id: String,
    pub node_id: String,
}

impl Config {
    pub fn load() -> Self {
        toml::from_str(&read_to_string("Config.toml").unwrap()).unwrap()
    }
}
