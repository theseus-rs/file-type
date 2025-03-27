use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1294021: FileType = FileType {
    file_format: &FileFormat {
        id: 1_294_021,
        source_type: SourceType::Wikidata,
        name: "OpenSearch",
        extensions: &[],
        media_types: &["application/opensearchdescription+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
