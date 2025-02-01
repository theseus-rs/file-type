use crate::format::FileFormat;

pub(crate) const LINGUIST_250: FileFormat = FileFormat {
    id: 250,
    puid: "linguist/250",
    name: "Ninja",
    extensions: &["ninja"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
