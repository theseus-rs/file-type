use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2463057504: FileType = FileType {
    file_format: &FileFormat {
        id: 2_463_057_504,
        source_type: SourceType::Iana,
        name: "trust-mark-status-response+jwt",
        extensions: &[],
        media_types: &["application/trust-mark-status-response+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
