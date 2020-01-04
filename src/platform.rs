use crate::config::PlatformsRolesConfig;
use serenity::model::id::RoleId;
use std::str::FromStr;
use crate::error::EnumParseError;

#[derive(Debug)]
pub enum Platform {
    IOS,
    Android
}

impl FromStr for Platform {
    type Err = EnumParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "ios" => Ok(Self::IOS),
            "android" => Ok(Self::Android),
            x => Err(Self::Err::VarientDoesNotExist(x.to_owned())),
        }
    }
}

impl Platform {
    pub fn get(roles: &[RoleId], config: &PlatformsRolesConfig) -> Option<Self> {
        for role in roles {
            match role {
                x if x == &config.ios => return Some(Self::IOS),
                x if x == &config.android => return Some(Self::Android),
                _ => {}
            }
        }

        None
    }
}
