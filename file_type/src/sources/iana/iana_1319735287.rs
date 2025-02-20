use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1319735287: FileType = FileType {
    file_format: &FileFormat {
        id: 1_319_735_287,
        source_type: SourceType::Iana,
        name: "sparql-results+xml",
        extensions: &[],
        media_types: &["application/sparql-results+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
