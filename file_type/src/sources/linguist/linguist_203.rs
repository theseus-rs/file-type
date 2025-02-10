use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_203: FileType = FileType {
    file_format: &FileFormat {
        id: 203,
        source_type: SourceType::Linguist,
        name: "Linux Kernel Module",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
