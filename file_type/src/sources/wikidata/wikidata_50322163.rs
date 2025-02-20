use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50322163: FileType = FileType {
    file_format: &FileFormat {
        id: 50_322_163,
        source_type: SourceType::Wikidata,
        name: "RDF/JSON",
        extensions: &["rj"],
        media_types: &["application/rdf+json"],
        signatures: &[],
        related_formats: &[],
    },
};
