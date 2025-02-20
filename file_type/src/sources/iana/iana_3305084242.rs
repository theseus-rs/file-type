use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3305084242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_305_084_242,
        source_type: SourceType::Iana,
        name: "dssc+xml",
        extensions: &[],
        media_types: &["application/dssc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
