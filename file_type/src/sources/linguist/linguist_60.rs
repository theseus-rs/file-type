use crate::format::FileFormat;

pub(crate) const LINGUIST_60: FileFormat = FileFormat {
    id: 60,
    puid: "linguist/60",
    name: "Clean",
    extensions: &["dcl", "icl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
