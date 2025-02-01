use crate::format::FileFormat;

pub(crate) const LINGUIST_179: FileFormat = FileFormat {
    id: 179,
    puid: "linguist/179",
    name: "Pug",
    extensions: &["jade", "pug"],
    media_types: &["text/x-pug"],
    internal_signatures: &[],
    related_formats: &[],
};
