use crate::format::FileFormat;

pub(crate) const LINGUIST_340: FileFormat = FileFormat {
    id: 340,
    puid: "linguist/340",
    name: "Sass",
    extensions: &["sass"],
    media_types: &["text/x-sass"],
    internal_signatures: &[],
    related_formats: &[],
};
