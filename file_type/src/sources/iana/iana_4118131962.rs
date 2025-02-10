use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4118131962: FileType = FileType {
    file_format: &FileFormat {
        id: 4_118_131_962,
        source_type: SourceType::Iana,
        name: "vnd.oma.poc.final-report+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.poc.final-report+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
