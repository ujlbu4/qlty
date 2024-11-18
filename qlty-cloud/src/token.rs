use anyhow::{bail, Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use keyring::Entry;

use crate::Client;

const KEYRING_SERVICE: &str = "qlty-cli";

pub struct Token {
    pub user: String,
}

impl Default for Token {
    fn default() -> Self {
        Self::new("default")
    }
}

impl Token {
    pub fn new(user: &str) -> Self {
        Self {
            user: user.to_owned(),
        }
    }

    pub fn get_with_interactive_prompt(&self) -> Result<String> {
        match self.get() {
            Ok(token) => Ok(token),
            Err(_) => {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("This action requires a CLI access token. Do you want to log in?")
                    .default(true)
                    .show_default(true)
                    .interact()?
                {
                    webbrowser::open("https://qlty.sh/user/settings/cli")?;

                    let access_token: String = Input::new()
                        .with_prompt("CLI access token")
                        .interact_text()
                        .unwrap();

                    let client = Client {
                        base_url: "https://api.qlty.sh".to_string(),
                        token: Some(access_token.clone()),
                    };

                    match client.get("/user").call() {
                        Ok(_) => {
                            self.set(&access_token)?;
                            Ok(access_token)
                        }
                        Err(e) => {
                            bail!("Failed to authenticate: {}", e);
                        }
                    }
                } else {
                    bail!("Please run `qlty auth login` to provide an access token to continue.")
                }
            }
        }
    }

    pub fn get(&self) -> Result<String> {
        self.keyring_entry()?.get_password().with_context(|| {
            format!(
                "Failed to get access token for service '{}' and user '{}'",
                KEYRING_SERVICE, self.user
            )
        })
    }

    pub fn set(&self, token: &str) -> Result<()> {
        self.keyring_entry()?.set_password(token).with_context(|| {
            format!(
                "Failed to set access token for service '{}' and user '{}'",
                KEYRING_SERVICE, self.user
            )
        })
    }

    pub fn delete(&self) -> Result<()> {
        self.keyring_entry()?.delete_credential().with_context(|| {
            format!(
                "Failed to delete access token for service '{}' and user '{}'",
                KEYRING_SERVICE, self.user
            )
        })
    }

    fn keyring_entry(&self) -> Result<Entry> {
        Entry::new(KEYRING_SERVICE, &self.user).with_context(|| {
            format!(
                "Failed to create keyring entry for service '{}' and user '{}'",
                KEYRING_SERVICE, self.user
            )
        })
    }
}
