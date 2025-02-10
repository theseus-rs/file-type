use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2009488547: FileType = FileType {
    file_format: &FileFormat {
        id: 2_009_488_547,
        source_type: SourceType::Iana,
        name: "sparql-query",
        extensions: &[],
        media_types: &["application/sparql-query"],
        signatures: &[],
        related_formats: &[],
    },
};
