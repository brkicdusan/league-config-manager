#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    WrongPath,
    MissingPath,
    NameTaken,
    ZipExport,
    ZipImport,
}
