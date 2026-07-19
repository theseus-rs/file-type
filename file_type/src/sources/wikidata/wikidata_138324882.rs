use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324882: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_882,
        source_type: SourceType::Wikidata,
        name: "SPARQL Results in TSV",
        extensions: &["tsv"],
        media_types: &["text/tab-separated-values"],
        signatures: &[],
        related_formats: &[],
    },
};
