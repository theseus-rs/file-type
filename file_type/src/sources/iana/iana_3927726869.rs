use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3927726869: FileType = FileType {
    file_format: &FileFormat {
        id: 3_927_726_869,
        source_type: SourceType::Iana,
        name: "framework-attributes+xml",
        extensions: &[],
        media_types: &["application/framework-attributes+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
