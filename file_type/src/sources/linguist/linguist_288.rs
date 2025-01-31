use crate::format::FileFormat;

pub(crate) const LINGUIST_288: FileFormat = FileFormat {
    id: 288,
    puid: "linguist/288",
    name: "Pod",
    extensions: &["pod"],
    media_types: &["text/x-perl"],
    internal_signatures: &[],
    related_formats: &[],
};
