use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_28923963: FileFormat = FileFormat {
    id: 28_923_963,
    source_type: SourceType::Linguist,
    name: "BASIC",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
