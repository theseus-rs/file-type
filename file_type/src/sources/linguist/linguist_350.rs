use crate::format::FileFormat;

pub(crate) const LINGUIST_350: FileFormat = FileFormat {
    id: 350,
    puid: "linguist/350",
    name: "Slim",
    extensions: &["slim"],
    media_types: &["text/x-slim"],
    internal_signatures: &[],
    related_formats: &[],
};
