use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3368099385: FileType = FileType {
    file_format: &FileFormat {
        id: 3_368_099_385,
        source_type: SourceType::Iana,
        name: "mathml+xml",
        extensions: &[],
        media_types: &["application/mathml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
