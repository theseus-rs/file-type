use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_4: FileFormat = FileFormat {
    id: 4,
    source_type: SourceType::Linguist,
    name: "ANTLR",
    extensions: &["g4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
