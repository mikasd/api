#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //init cfg reader
    let mut settings = config::Config::default();

    //add cfg vals from file named configuration
    //crate will look for any top level file w parse-able extension yaml/json/etc
    settings.merge(config::File::with_name("configuration"))?;
    //try to convert cfg vals into settings type
    settings.try_into()
}