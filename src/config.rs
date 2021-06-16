use std::env;

#[derive(Debug)]
pub struct Configuration {
    pub service_name: String,
    pub listen_on: String,
    pub www_folder: String,
}

#[cfg(debug_assertions)]
fn expect_env_var(name: &str, default: &str) -> String {
    return env::var(name).unwrap_or(String::from(default));
}

#[cfg(not(debug_assertions))]
fn expect_env_var(name: &str, _default: &str) -> String {
    return env::var(name).expect(&format!(
        "Environment variable {name} is not defined",
        name = name
    ));
}

pub fn get_configuration() -> Configuration {
    Configuration {
        service_name: expect_env_var("SERVICE_NAME", "wwwloader"),
        listen_on: expect_env_var("LISTEN_ON", "0.0.0.0:3030"),
        www_folder: expect_env_var("WWW_FOLDER", "www"),
    }
}
