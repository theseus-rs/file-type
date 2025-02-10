use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1679: FileType = FileType {
    file_format: &FileFormat {
        id: 1_679,
        source_type: SourceType::Pronom,
        name: "RDF/XML",
        extensions: &["rdf"],
        media_types: &["application/rdf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
