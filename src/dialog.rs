use crate::error::Error;

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
