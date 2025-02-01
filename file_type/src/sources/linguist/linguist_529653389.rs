use crate::format::FileFormat;

pub(crate) const LINGUIST_529653389: FileFormat = FileFormat {
    id: 529_653_389,
    puid: "linguist/529653389",
    name: "E-mail",
    extensions: &["eml", "mbox"],
    media_types: &["application/mbox"],
    internal_signatures: &[],
    related_formats: &[],
};
