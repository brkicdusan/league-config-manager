use crate::{error::Error, profile::Profile};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    FindLocation,
    SetLocation(Result<PathBuf, Error>),
    SetReadonly(bool),
    AddProfile,
    RemoveProfile(String),
    UseProfile(Profile),
    Edit(String),
    Confirm(String),
    Reset(String),
    OnChange(String, String),
    Export(Profile),
    Import(PathBuf),
}
