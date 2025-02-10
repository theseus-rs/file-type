use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_668457123: FileType = FileType {
    file_format: &FileFormat {
        id: 668_457_123,
        source_type: SourceType::Linguist,
        name: "Wget Config",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
