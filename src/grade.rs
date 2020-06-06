use crate::config::GradesRolesConfig;
use crate::error::EnumParseError;
use serenity::model::id::RoleId;
use std::str::FromStr;

#[derive(Debug)]
pub enum Grade {
    Freshman,
    Sophomore,
    Junior,
    Senior,
    Graduate,
}

impl FromStr for Grade {
    type Err = EnumParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "freshman" => Ok(Self::Freshman),
            "sophomore" => Ok(Self::Sophomore),
            "junior" => Ok(Self::Junior),
            "sernior" => Ok(Self::Senior),
            "graduate" => Ok(Self::Graduate),
            x => Err(Self::Err::VarientDoesNotExist(x.to_owned())),
        }
    }
}

impl Grade {
    pub fn get(roles: &[RoleId], config: &GradesRolesConfig) -> Option<Grade> {
        for role in roles {
            match role {
                x if x == &config.freshman => return Some(Grade::Freshman),
                x if x == &config.sophomore => return Some(Grade::Sophomore),
                x if x == &config.junior => return Some(Grade::Junior),
                x if x == &config.senior => return Some(Grade::Senior),
                x if x == &config.graduate => return Some(Grade::Graduate),
                _ => {}
            }
        }

        None
    }
}
