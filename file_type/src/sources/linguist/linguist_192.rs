use crate::format::FileFormat;

pub(crate) const LINGUIST_192: FileFormat = FileFormat {
    id: 192,
    puid: "linguist/192",
    name: "LOLCODE",
    extensions: &["lol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
