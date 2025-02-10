use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_77: FileType = FileType {
    file_format: &FileFormat {
        id: 77,
        source_type: SourceType::Linguist,
        name: "Cuda",
        extensions: &["cu", "cuh"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
