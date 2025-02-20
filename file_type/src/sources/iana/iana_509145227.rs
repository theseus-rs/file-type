use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_509145227: FileType = FileType {
    file_format: &FileFormat {
        id: 509_145_227,
        source_type: SourceType::Iana,
        name: "pidf+xml",
        extensions: &[],
        media_types: &["application/pidf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
