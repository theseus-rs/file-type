use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_47: FileType = FileType {
    file_format: &FileFormat {
        id: 47,
        source_type: SourceType::Linguist,
        name: "CMake",
        extensions: &["cmake", "cmake.in"],
        media_types: &["text/x-cmake"],
        signatures: &[],
        related_formats: &[],
    },
};
