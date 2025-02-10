use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1721268295: FileType = FileType {
    file_format: &FileFormat {
        id: 1_721_268_295,
        source_type: SourceType::Iana,
        name: "metalink4+xml",
        extensions: &[],
        media_types: &["application/metalink4+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
