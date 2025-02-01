use crate::format::FileFormat;

pub(crate) const LINGUIST_334: FileFormat = FileFormat {
    id: 334,
    puid: "linguist/334",
    name: "SQLPL",
    extensions: &["db2", "sql"],
    media_types: &["text/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
