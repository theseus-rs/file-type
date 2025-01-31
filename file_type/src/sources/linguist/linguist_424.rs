use crate::format::FileFormat;

pub(crate) const LINGUIST_424: FileFormat = FileFormat {
    id: 424,
    puid: "linguist/424",
    name: "CSON",
    extensions: &["cson"],
    media_types: &["text/x-coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
