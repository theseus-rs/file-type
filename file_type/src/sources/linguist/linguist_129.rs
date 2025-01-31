use crate::format::FileFormat;

pub(crate) const LINGUIST_129: FileFormat = FileFormat {
    id: 129,
    puid: "linguist/129",
    name: "Gettext Catalog",
    extensions: &["po", "pot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
