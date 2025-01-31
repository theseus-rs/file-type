use crate::format::FileFormat;

pub(crate) const LINGUIST_88: FileFormat = FileFormat {
    id: 88,
    puid: "linguist/88",
    name: "Diff",
    extensions: &["diff", "patch"],
    media_types: &["text/x-diff"],
    internal_signatures: &[],
    related_formats: &[],
};
