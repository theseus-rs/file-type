use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_833504686: FileType = FileType {
    file_format: &FileFormat {
        id: 833_504_686,
        source_type: SourceType::Linguist,
        name: "PlantUML",
        extensions: &["iuml", "plantuml", "puml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
