use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1210000402: FileType = FileType {
    file_format: &FileFormat {
        id: 1_210_000_402,
        source_type: SourceType::Iana,
        name: "vnd.capasystems-pg+json",
        extensions: &[],
        media_types: &["application/vnd.capasystems-pg+json"],
        signatures: &[],
        related_formats: &[],
    },
};
