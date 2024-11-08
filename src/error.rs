#[derive(Debug, Clone, Copy)]
pub enum Error {
    DialogClosed,
    WrongPath,
    MissingPath,
    NameTaken,
    ZipExport,
    ZipImport,
    ChampionTaken,
    Import,
}
