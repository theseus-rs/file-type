use crate::format::FileFormat;

pub(crate) const LINGUIST_41: FileFormat = FileFormat {
    id: 41,
    puid: "linguist/41",
    name: "C",
    extensions: &["c", "cats", "h", "idc"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
