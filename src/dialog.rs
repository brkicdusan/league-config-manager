use crate::{error::Error, profile::Profile};

use std::path::PathBuf;

/// Opens dialog to locate config.
///
/// # Errors
/// When dialog closes return Error::DialogClosed
pub(crate) async fn find_config_dialog() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Find \"League of Legends\" folder")
        .pick_folder()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(handle.path().to_owned())
}

pub(crate) async fn import_zip_path() -> Result<PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose profile zip to import")
        .add_filter("zip", &["zip"])
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(handle.path().to_owned())
}

pub(crate) async fn export_zip_path(profile: Profile) -> Result<(PathBuf, Profile), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose folder to export to")
        .pick_folder()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok((handle.path().to_owned(), profile))
}
