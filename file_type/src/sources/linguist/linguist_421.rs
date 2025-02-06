use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_421: FileFormat = FileFormat {
    id: 421,
    source_type: SourceType::Linguist,
    name: "xBase",
    extensions: &["ch", "prg", "prw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
