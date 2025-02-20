use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_331: FileType = FileType {
    file_format: &FileFormat {
        id: 331,
        source_type: SourceType::Linguist,
        name: "SPARQL",
        extensions: &["rq", "sparql"],
        media_types: &["application/sparql-query"],
        signatures: &[],
        related_formats: &[],
    },
};
