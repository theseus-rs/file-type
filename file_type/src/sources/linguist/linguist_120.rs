use crate::format::FileFormat;

pub(crate) const LINGUIST_120: FileFormat = FileFormat {
    id: 120,
    puid: "linguist/120",
    name: "Unix Assembly",
    extensions: &["ms", "s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
