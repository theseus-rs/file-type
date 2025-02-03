use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_331: FileFormat = FileFormat {
    id: 331,
    source_type: SourceType::Linguist,
    name: "SPARQL",
    extensions: &["rq", "sparql"],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
