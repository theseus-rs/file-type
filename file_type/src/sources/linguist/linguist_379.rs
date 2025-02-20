use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_379: FileType = FileType {
    file_format: &FileFormat {
        id: 379,
        source_type: SourceType::Linguist,
        name: "Unified Parallel C",
        extensions: &["upc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
