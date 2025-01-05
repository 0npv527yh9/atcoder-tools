use crate::service::login::{Error, LoginService};

pub fn login(login_service: LoginService) -> Result<(), Error> {
    loop {
        let username = rprompt::prompt_reply("Username:").map_err(Error::IOError)?;
        let password = rpassword::prompt_password("Password:").map_err(Error::IOError)?;

        match login_service.login(&username, &password) {
            Ok(_) => {
                println!("Login Successs");
                login_service.save_session_data().map_err(Error::IOError)?;
                return Ok(());
            }
            Err(e) => {
                let message = rprompt::prompt_reply("Retry? ([y]/n):")
                    .map_err(Error::IOError)?
                    .to_lowercase();
                let should_retry = message == "y" || message == "yes";
                if !should_retry {
                    return Err(e);
                }
            }
        }
    }
}
