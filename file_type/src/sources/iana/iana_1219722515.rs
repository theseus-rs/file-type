use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1219722515: FileType = FileType {
    file_format: &FileFormat {
        id: 1_219_722_515,
        source_type: SourceType::Iana,
        name: "xcap-ns+xml",
        extensions: &[],
        media_types: &["application/xcap-ns+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
