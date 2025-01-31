use crate::format::FileFormat;

pub(crate) const LINGUIST_175: FileFormat = FileFormat {
    id: 175,
    puid: "linguist/175",
    name: "JSON5",
    extensions: &["json5"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
