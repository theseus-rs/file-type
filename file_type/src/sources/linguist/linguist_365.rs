use crate::format::FileFormat;

pub(crate) const LINGUIST_365: FileFormat = FileFormat {
    id: 365,
    puid: "linguist/365",
    name: "TOML",
    extensions: &["toml"],
    media_types: &["text/x-toml"],
    internal_signatures: &[],
    related_formats: &[],
};
