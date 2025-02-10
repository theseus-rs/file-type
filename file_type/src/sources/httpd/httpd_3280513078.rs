use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3280513078: FileType = FileType {
    file_format: &FileFormat {
        id: 3_280_513_078,
        source_type: SourceType::Httpd,
        name: "turtle",
        extensions: &["ttl"],
        media_types: &["text/turtle"],
        signatures: &[],
        related_formats: &[],
    },
};
