use crate::format::FileFormat;

pub(crate) const LINGUIST_203: FileFormat = FileFormat {
    id: 203,
    puid: "linguist/203",
    name: "Linux Kernel Module",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
