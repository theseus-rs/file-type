use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2500032212: FileType = FileType {
    file_format: &FileFormat {
        id: 2_500_032_212,
        source_type: SourceType::Iana,
        name: "vnd.geocube+xml (OBSOLETED by request)",
        extensions: &[],
        media_types: &["application/vnd.geocube+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
