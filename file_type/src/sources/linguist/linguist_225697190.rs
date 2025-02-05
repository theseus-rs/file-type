use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_225697190: FileFormat = FileFormat {
    id: 225_697_190,
    source_type: SourceType::Linguist,
    name: "Kusto",
    extensions: &["csl", "kql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
