use crate::format::FileFormat;

pub(crate) const LINGUIST_425: FileFormat = FileFormat {
    id: 425,
    puid: "linguist/425",
    name: "Pic",
    extensions: &["chem", "pic"],
    media_types: &["text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
