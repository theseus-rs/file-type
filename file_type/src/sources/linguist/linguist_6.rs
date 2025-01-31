use crate::format::FileFormat;

pub(crate) const LINGUIST_6: FileFormat = FileFormat {
    id: 6,
    puid: "linguist/6",
    name: "APL",
    extensions: &["apl", "dyalog"],
    media_types: &["text/apl"],
    internal_signatures: &[],
    related_formats: &[],
};
