use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_2: FileType = FileType {
    file_format: &FileFormat {
        id: 2,
        source_type: SourceType::Linguist,
        name: "AGS Script",
        extensions: &["asc", "ash"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
