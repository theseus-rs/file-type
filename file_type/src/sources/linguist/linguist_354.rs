use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_354: FileFormat = FileFormat {
    id: 354,
    source_type: SourceType::Linguist,
    name: "SourcePawn",
    extensions: &["inc", "sp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
