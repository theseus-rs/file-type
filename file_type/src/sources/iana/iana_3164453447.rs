use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3164453447: FileType = FileType {
    file_format: &FileFormat {
        id: 3_164_453_447,
        source_type: SourceType::Iana,
        name: "rlmi+xml",
        extensions: &[],
        media_types: &["application/rlmi+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
