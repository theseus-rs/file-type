use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2453250016: FileType = FileType {
    file_format: &FileFormat {
        id: 2_453_250_016,
        source_type: SourceType::Iana,
        name: "kpml-request+xml",
        extensions: &[],
        media_types: &["application/kpml-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
