mod auth_flow;

use crate::Client;
use anyhow::{Context, Result};
use keyring::Entry;
use tracing::warn;

pub const SERVICE: &str = "qlty-cli";
pub const DEFAULT_USER: &str = "default";

pub struct Token {
    pub user: String,
}

impl Default for Token {
    fn default() -> Self {
        Self::new(DEFAULT_USER)
    }
}

impl Token {
    pub fn new(user: &str) -> Self {
        Self {
            user: user.to_owned(),
        }
    }

    pub fn get(&self) -> Result<String> {
        self.entry()?.get_password().with_context(|| {
            format!(
                "Failed to get access token for service '{}' and user '{}'",
                SERVICE, self.user
            )
        })
    }

    pub fn set(&self, token: &str) -> Result<()> {
        self.entry()?.set_password(token).with_context(|| {
            format!(
                "Failed to set access token for service '{}' and user '{}'",
                SERVICE, self.user
            )
        })
    }

    pub fn delete(&self) -> Result<()> {
        self.entry()?.delete_credential().with_context(|| {
            format!(
                "Failed to delete access token for service '{}' and user '{}'",
                SERVICE, self.user
            )
        })
    }

    fn entry(&self) -> Result<Entry> {
        Entry::new(SERVICE, &self.user).with_context(|| {
            format!(
                "Failed to create keyring entry for service '{}' and user '{}'",
                SERVICE, self.user
            )
        })
    }
}

pub fn load_auth_token() -> Result<String> {
    let mut has_token = false;
    let auth_token = match Token::default().get() {
        Ok(token) => {
            has_token = true;
            Ok(token)
        }
        Err(_) => auth_flow::auth_via_browser(),
    }?;

    match validate_auth_token(&auth_token) {
        Ok(_) => Ok(auth_token),
        Err(err) => {
            if has_token {
                warn!("Failed to validate existing auth token, attempting to re-authenticate");
                load_auth_token()
            } else {
                Err(err)
            }
        }
    }
}

pub fn clear_auth_token() -> Result<()> {
    Token::default().delete()
}

fn validate_auth_token(auth_token: &String) -> Result<()> {
    Client::new(None, Some(auth_token.into()))
        .get("/user")
        .call()
        .map_err(|client_err| {
            if let Err(err) = clear_auth_token() {
                warn!("Failed to clear auth token: {}", err);
            }
            client_err
        })?;

    Ok(())
}
