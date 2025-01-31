use crate::format::FileFormat;

pub(crate) const LINGUIST_113: FileFormat = FileFormat {
    id: 113,
    puid: "linguist/113",
    name: "Formatted",
    extensions: &["eam.fs", "for"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
