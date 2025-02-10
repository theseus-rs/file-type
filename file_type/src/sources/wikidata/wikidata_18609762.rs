use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_18609762: FileType = FileType {
    file_format: &FileFormat {
        id: 18_609_762,
        source_type: SourceType::Wikidata,
        name: "SPARQL Query Results JSON Format",
        extensions: &["srj"],
        media_types: &["application/sparql-results+json"],
        signatures: &[],
        related_formats: &[],
    },
};
