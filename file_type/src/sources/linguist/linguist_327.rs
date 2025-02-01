use crate::format::FileFormat;

pub(crate) const LINGUIST_327: FileFormat = FileFormat {
    id: 327,
    puid: "linguist/327",
    name: "Rust",
    extensions: &["rs", "rs.in"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
