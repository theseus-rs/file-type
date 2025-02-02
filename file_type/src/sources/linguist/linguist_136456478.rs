use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_136456478: FileFormat = FileFormat {
    id: 136_456_478,
    source_type: SourceType::Linguist,
    name: "NMODL",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
