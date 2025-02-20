use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44044: FileType = FileType {
    file_format: &FileFormat {
        id: 44_044,
        source_type: SourceType::Wikidata,
        name: "N-Triples",
        extensions: &["nt"],
        media_types: &["application/n-triples"],
        signatures: &[],
        related_formats: &[],
    },
};
