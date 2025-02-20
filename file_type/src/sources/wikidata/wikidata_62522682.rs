use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62522682: FileType = FileType {
    file_format: &FileFormat {
        id: 62_522_682,
        source_type: SourceType::Wikidata,
        name: "SPARQL update",
        extensions: &["ru"],
        media_types: &["application/sparql-update"],
        signatures: &[],
        related_formats: &[],
    },
};
