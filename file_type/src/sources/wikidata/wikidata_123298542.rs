use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123298542: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_542,
        source_type: SourceType::Wikidata,
        name: "Retrospect RBF File",
        extensions: &["rbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
