use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_432600901: FileType = FileType {
    file_format: &FileFormat {
        id: 432_600_901,
        source_type: SourceType::Linguist,
        name: "Gradle Kotlin DSL",
        extensions: &["gradle.kts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
