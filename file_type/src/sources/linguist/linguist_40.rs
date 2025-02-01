use crate::format::FileFormat;

pub(crate) const LINGUIST_40: FileFormat = FileFormat {
    id: 40,
    puid: "linguist/40",
    name: "Zeek",
    extensions: &["bro", "zeek"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
