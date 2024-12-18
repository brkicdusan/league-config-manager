use crate::{error::Error, profile::Profile, websocket};

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    FindLocation,
    SetLocation(Result<PathBuf, Error>),
    SetReadonly(bool),

    //Profile
    AddProfile,
    RemoveProfile(String),
    UseProfile(Profile),
    Edit(String),
    Confirm(String),
    Reset(String),
    OnChange(String, String),
    Export(Profile),
    SetExport(Result<(PathBuf, Profile), Error>),
    Import,
    SetImport(Result<PathBuf, Error>),
    PickListChange(String, &'static str),

    CopyLink(String),
    GenerateLink(String, String),
    PostLink(String, String),
    PostLinkError(String),

    ChangeLink(String),

    FetchLink(String),
    AddProfileFromString(String),
    FetchError(Error),

    WebsocketEvent(websocket::Event),
}
