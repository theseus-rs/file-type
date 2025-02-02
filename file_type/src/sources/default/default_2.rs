use crate::format::{FileFormat, SourceType};

pub(crate) const DEFAULT_2: FileFormat = FileFormat {
    id: 2,
    source_type: SourceType::Default,
    name: "Text",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
