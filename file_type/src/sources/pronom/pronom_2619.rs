use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2619: FileType = FileType {
    file_format: &FileFormat {
        id: 2_619,
        source_type: SourceType::Pronom,
        name: "C++ Source Code File",
        extensions: &["cpp", "cxx", "cc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
