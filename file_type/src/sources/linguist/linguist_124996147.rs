use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_124996147: FileType = FileType {
    file_format: &FileFormat {
        id: 124_996_147,
        source_type: SourceType::Linguist,
        name: "ASL",
        extensions: &["asl", "dsl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
