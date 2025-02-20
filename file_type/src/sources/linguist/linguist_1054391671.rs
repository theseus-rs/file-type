use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1054391671: FileType = FileType {
    file_format: &FileFormat {
        id: 1_054_391_671,
        source_type: SourceType::Linguist,
        name: "Go Checksums",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
