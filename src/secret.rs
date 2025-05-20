use secrecy::SecretString;

/// The secret enum.
///
/// A secret can be retrieved either from a raw string, from a shell
/// command or from a keyring entry.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "serde", serde(try_from = "crate::serde::Secret"))]
#[cfg_attr(feature = "serde", serde(into = "crate::serde::Secret"))]
pub enum Secret {
    /// The secret is contained in a raw string.
    ///
    /// This variant is not safe to use and therefore not
    /// recommended. Yet it works well for testing purpose.
    Raw(SecretString),

    /// The secret is exposed by the given shell command.
    ///
    /// This variant takes the secret from the first line returned by
    /// the given shell command.
    ///
    /// See [process-flows](https://crates.io/crates/process-flows).
    #[cfg(feature = "command")]
    Command(io_process::Command),

    /// The secret is contained in the user's global keyring at the
    /// given entry.
    ///
    /// See [keyring-flows](https://crates.io/crates/keyring-flows).
    #[cfg(feature = "keyring")]
    Keyring(io_keyring::Entry),
}

impl Secret {
    // /// Gets the secret value.
    // ///
    // /// The command-based secret execute its shell command and returns
    // /// the output, and the keyring-based secret retrieves the value
    // /// from the global keyring using its inner key.
    // pub async fn get(&self) -> Result<String> {
    //     match self {
    //         Self::Empty => {
    //             return Err(Error::GetEmptySecretError);
    //         }
    //         Self::Raw(secret) => {
    //             return Ok(secret.clone());
    //         }
    //         #[cfg(feature = "command")]
    //         Self::Command(cmd) => {
    //             let full_secret = cmd
    //                 .run()
    //                 .await
    //                 .map_err(Error::GetSecretFromCommand)?
    //                 .to_string_lossy();

    //             let first_line_secret = full_secret
    //                 .lines()
    //                 .take(1)
    //                 .next()
    //                 .ok_or(Error::GetSecretFromCommandEmptyOutputError)?
    //                 .to_owned();

    //             Ok(first_line_secret)
    //         }
    //         #[cfg(feature = "keyring")]
    //         Self::Keyring(entry) => {
    //             let secret = entry.get_secret().await?;
    //             Ok(secret)
    //         }
    //     }
    // }

    // /// Finds the secret value.
    // ///
    // /// Like [`Secret::get`], but returns [`None`] if the secret value
    // /// is not found or empty.
    // pub async fn find(&self) -> Result<Option<String>> {
    //     match self {
    //         Self::Empty => {
    //             return Ok(None);
    //         }
    //         Self::Raw(secret) => {
    //             return Ok(Some(secret.clone()));
    //         }
    //         #[cfg(feature = "command")]
    //         Self::Command(cmd) => {
    //             let full_secret = cmd
    //                 .run()
    //                 .await
    //                 .map_err(Error::GetSecretFromCommand)?
    //                 .to_string_lossy();

    //             let first_line_secret = full_secret.lines().take(1).next().map(ToOwned::to_owned);

    //             Ok(first_line_secret)
    //         }
    //         #[cfg(feature = "keyring")]
    //         Self::Keyring(entry) => {
    //             let secret = entry.find_secret().await?;
    //             Ok(secret)
    //         }
    //     }
    // }

    // /// Updates the secret value.
    // ///
    // /// This is only applicable for raw secrets and keyring-based
    // /// secrets. A secret value cannot be changed for command-base
    // /// secrets, since the value is the output of the command.
    // pub async fn set(&mut self, secret: impl ToString) -> Result<String> {
    //     match self {
    //         Self::Raw(prev) => {
    //             *prev = secret.to_string();
    //         }
    //         #[cfg(feature = "command")]
    //         Self::Command(_) => {
    //             debug!("cannot change value of command-based secret");
    //         }
    //         #[cfg(feature = "keyring")]
    //         Self::Keyring(entry) => entry.set_secret(secret.to_string()).await?,
    //         Self::Empty => {
    //             debug!("cannot change value of empty secret");
    //         }
    //     }

    //     Ok(secret.to_string())
    // }

    // /// Updates the secret value of the keyring-based secret only.
    // ///
    // /// This function as no effect on other secret variants.
    // #[cfg(feature = "keyring")]
    // pub async fn set_if_keyring(&self, secret: impl ToString) -> Result<String> {
    //     if let Self::Keyring(entry) = self {
    //         let secret = secret.to_string();
    //         entry.set_secret(&secret).await?;
    //         return Ok(secret);
    //     }

    //     Ok(secret.to_string())
    // }

    // /// Deletes the secret value and make the current secret empty.
    // pub async fn delete(&mut self) -> Result<()> {
    //     #[cfg(feature = "keyring")]
    //     if let Self::Keyring(entry) = self {
    //         entry.delete_secret().await?;
    //     }

    //     *self = Self::Empty;

    //     Ok(())
    // }

    // /// Deletes the secret value of keyring-based secrets only.
    // ///
    // /// This function has no effect on other variants.
    // #[cfg(feature = "keyring")]
    // pub async fn delete_if_keyring(&self) -> Result<()> {
    //     if let Self::Keyring(entry) = self {
    //         entry.delete_secret().await?;
    //     }

    //     Ok(())
    // }

    // /// Replaces empty secret variant with the given one.
    // ///
    // /// This function has no effect on other variants.
    // pub fn replace_if_empty(&mut self, new: Self) {
    //     if self.is_empty() {
    //         *self = new
    //     }
    // }

    // /// Replaces empty secret variant with a keyring one.
    // ///
    // /// This function has no effect on other variants.
    // #[cfg(feature = "keyring")]
    // pub fn replace_with_keyring_if_empty(&mut self, entry: impl ToString) -> Result<()> {
    //     if self.is_empty() {
    //         *self = Self::try_new_keyring_entry(entry.to_string())?;
    //     }

    //     Ok(())
    // }
}
