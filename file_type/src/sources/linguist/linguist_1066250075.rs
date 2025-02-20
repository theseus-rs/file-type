use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1066250075: FileType = FileType {
    file_format: &FileFormat {
        id: 1_066_250_075,
        source_type: SourceType::Linguist,
        name: "SWIG",
        extensions: &["i"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
