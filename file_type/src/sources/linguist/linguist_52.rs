use crate::format::FileFormat;

pub(crate) const LINGUIST_52: FileFormat = FileFormat {
    id: 52,
    puid: "linguist/52",
    name: "Cap'n Proto",
    extensions: &["capnp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
