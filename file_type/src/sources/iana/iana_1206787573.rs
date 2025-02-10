use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1206787573: FileType = FileType {
    file_format: &FileFormat {
        id: 1_206_787_573,
        source_type: SourceType::Iana,
        name: "vnd.nokia.landmark+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.landmark+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
