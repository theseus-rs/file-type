use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1393245079: FileType = FileType {
    file_format: &FileFormat {
        id: 1_393_245_079,
        source_type: SourceType::Iana,
        name: "vnd.phbk+xml",
        extensions: &[],
        media_types: &["application/vnd.phbk+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
