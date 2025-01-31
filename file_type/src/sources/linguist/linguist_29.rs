use crate::format::FileFormat;

pub(crate) const LINGUIST_29: FileFormat = FileFormat {
    id: 29,
    puid: "linguist/29",
    name: "Batchfile",
    extensions: &["bat", "cmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
