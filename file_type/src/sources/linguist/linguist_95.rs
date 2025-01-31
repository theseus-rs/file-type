use crate::format::FileFormat;

pub(crate) const LINGUIST_95: FileFormat = FileFormat {
    id: 95,
    puid: "linguist/95",
    name: "EJS",
    extensions: &["ect", "ejs", "ejs.t", "jst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
