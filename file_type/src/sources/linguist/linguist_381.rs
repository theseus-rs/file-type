use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_381: FileType = FileType {
    file_format: &FileFormat {
        id: 381,
        source_type: SourceType::Linguist,
        name: "Uno",
        extensions: &["uno"],
        media_types: &["text/x-csharp"],
        signatures: &[],
        related_formats: &[],
    },
};
