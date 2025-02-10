use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_189: FileType = FileType {
    file_format: &FileFormat {
        id: 189,
        source_type: SourceType::Linguist,
        name: "Kotlin",
        extensions: &["kt", "ktm", "kts"],
        media_types: &["text/x-kotlin"],
        signatures: &[],
        related_formats: &[],
    },
};
