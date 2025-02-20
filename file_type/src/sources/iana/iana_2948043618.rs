use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2948043618: FileType = FileType {
    file_format: &FileFormat {
        id: 2_948_043_618,
        source_type: SourceType::Iana,
        name: "fastsoap",
        extensions: &[],
        media_types: &["application/fastsoap"],
        signatures: &[],
        related_formats: &[],
    },
};
