use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_805122868: FileType = FileType {
    file_format: &FileFormat {
        id: 805_122_868,
        source_type: SourceType::Linguist,
        name: "YARA",
        extensions: &["yar", "yara"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
