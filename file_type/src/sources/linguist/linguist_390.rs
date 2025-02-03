use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_390: FileFormat = FileFormat {
    id: 390,
    source_type: SourceType::Linguist,
    name: "Volt",
    extensions: &["volt"],
    media_types: &["text/x-d"],
    internal_signatures: &[],
    related_formats: &[],
};
