use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48940: FileType = FileType {
    file_format: &FileFormat {
        id: 48_940,
        source_type: SourceType::Wikidata,
        name: "RDF/XML",
        extensions: &["rdf"],
        media_types: &["application/rdf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
