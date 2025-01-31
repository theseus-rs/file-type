use crate::format::FileFormat;

pub(crate) const LINGUIST_99: FileFormat = FileFormat {
    id: 99,
    puid: "linguist/99",
    name: "Eiffel",
    extensions: &["e"],
    media_types: &["text/x-eiffel"],
    internal_signatures: &[],
    related_formats: &[],
};
