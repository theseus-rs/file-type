use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_394: FileFormat = FileFormat {
    id: 394,
    source_type: SourceType::Linguist,
    name: "Web Ontology Language",
    extensions: &["owl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
