use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_826165: FileType = FileType {
    file_format: &FileFormat {
        id: 826_165,
        source_type: SourceType::Wikidata,
        name: "Web Ontology Language",
        extensions: &["owl"],
        media_types: &["application/owl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
