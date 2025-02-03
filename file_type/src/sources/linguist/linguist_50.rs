use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_50: FileFormat = FileFormat {
    id: 50,
    source_type: SourceType::Linguist,
    name: "CSS",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
