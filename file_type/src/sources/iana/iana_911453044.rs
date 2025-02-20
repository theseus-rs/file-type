use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_911453044: FileType = FileType {
    file_format: &FileFormat {
        id: 911_453_044,
        source_type: SourceType::Iana,
        name: "vnd.biopax.rdf+xml",
        extensions: &[],
        media_types: &["application/vnd.biopax.rdf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
