use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_18609754: FileType = FileType {
    file_format: &FileFormat {
        id: 18_609_754,
        source_type: SourceType::Wikidata,
        name: "SPARQL Query Results XML Format",
        extensions: &["srx"],
        media_types: &["application/sparql-results+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
