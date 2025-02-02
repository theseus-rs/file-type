use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_664885656: FileFormat = FileFormat {
    id: 664_885_656,
    source_type: SourceType::Linguist,
    name: "Jsonnet",
    extensions: &["jsonnet", "libsonnet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
