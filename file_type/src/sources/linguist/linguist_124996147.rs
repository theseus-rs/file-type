use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_124996147: FileFormat = FileFormat {
    id: 124_996_147,
    source_type: SourceType::Linguist,
    name: "ASL",
    extensions: &["asl", "dsl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
