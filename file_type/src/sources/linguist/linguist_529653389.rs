use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_529653389: FileFormat = FileFormat {
    id: 529_653_389,
    source_type: SourceType::Linguist,
    name: "E-mail",
    extensions: &["eml", "mbox"],
    media_types: &["application/mbox"],
    signatures: &[],
    related_formats: &[],
};
