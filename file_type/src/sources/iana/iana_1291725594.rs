use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1291725594: FileType = FileType {
    file_format: &FileFormat {
        id: 1_291_725_594,
        source_type: SourceType::Iana,
        name: "vec+xml",
        extensions: &[],
        media_types: &["application/vec+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
