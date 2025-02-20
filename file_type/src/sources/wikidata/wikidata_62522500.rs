use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62522500: FileType = FileType {
    file_format: &FileFormat {
        id: 62_522_500,
        source_type: SourceType::Wikidata,
        name: "SPARQL query file format",
        extensions: &["rq"],
        media_types: &["application/sparql-query"],
        signatures: &[],
        related_formats: &[],
    },
};
