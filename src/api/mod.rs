use std::env;

use crate::domain::constants;

pub mod controllers;
pub mod dto;

fn get_jwt_secret() -> String {
    env::var(constants::JWT_SECRET).unwrap_or("verysecret".to_string())
}
