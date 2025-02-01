use crate::format::FileFormat;

pub(crate) const LINGUIST_311: FileFormat = FileFormat {
    id: 311,
    puid: "linguist/311",
    name: "REXX",
    extensions: &["pprx", "rex", "rexx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
