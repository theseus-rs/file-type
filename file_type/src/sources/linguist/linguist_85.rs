use crate::format::FileFormat;

pub(crate) const LINGUIST_85: FileFormat = FileFormat {
    id: 85,
    puid: "linguist/85",
    name: "DTrace",
    extensions: &["d"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
