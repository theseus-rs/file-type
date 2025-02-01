use crate::format::FileFormat;

pub(crate) const LINGUIST_411: FileFormat = FileFormat {
    id: 411,
    puid: "linguist/411",
    name: "Zimpl",
    extensions: &["zimpl", "zmpl", "zpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
