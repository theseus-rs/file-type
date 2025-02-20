use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
