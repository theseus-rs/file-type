use crate::format::FileFormat;

pub(crate) const LINGUIST_421: FileFormat = FileFormat {
    id: 421,
    puid: "linguist/421",
    name: "xBase",
    extensions: &["ch", "prg", "prw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
