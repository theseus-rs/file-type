use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_108802647: FileType = FileType {
    file_format: &FileFormat {
        id: 108_802_647,
        source_type: SourceType::Iana,
        name: "xhtml+xml",
        extensions: &[],
        media_types: &["application/xhtml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
