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
}
