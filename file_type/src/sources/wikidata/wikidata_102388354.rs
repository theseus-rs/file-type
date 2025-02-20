use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_102388354: FileType = FileType {
    file_format: &FileFormat {
        id: 102_388_354,
        source_type: SourceType::Wikidata,
        name: "SPARQL-Generate",
        extensions: &["rqg"],
        media_types: &["application/vnd.sparql-generate"],
        signatures: &[],
        related_formats: &[],
    },
};
