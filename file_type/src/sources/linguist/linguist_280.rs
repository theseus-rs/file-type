use crate::format::FileFormat;

pub(crate) const LINGUIST_280: FileFormat = FileFormat {
    id: 280,
    puid: "linguist/280",
    name: "Parrot Internal Representation",
    extensions: &["pir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
