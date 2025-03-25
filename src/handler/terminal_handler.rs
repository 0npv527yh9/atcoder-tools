use std::io::Result;

pub fn read_credentials() -> Result<Credentials> {
    let username = rprompt::prompt_reply("Username:")?;
    let password = rpassword::prompt_password("Password:")?;
    Ok(Credentials { username, password })
}

pub fn ask_for_retry() -> Result<bool> {
    let input = rprompt::prompt_reply("Retry? (y/[n]):")?.to_lowercase();
    Ok(&input == "y" || &input == "yes")
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}
