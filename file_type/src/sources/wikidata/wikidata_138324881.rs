use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324881: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_881,
        source_type: SourceType::Wikidata,
        name: "SPARQL Results in CSV",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[],
        related_formats: &[],
    },
};
