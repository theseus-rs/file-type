use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_51601661: FileFormat = FileFormat {
    id: 51_601_661,
    source_type: SourceType::Linguist,
    name: "Rich Text Format",
    extensions: &["rtf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
