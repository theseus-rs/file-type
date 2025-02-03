use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_136: FileFormat = FileFormat {
    id: 136,
    source_type: SourceType::Linguist,
    name: "Gradle",
    extensions: &["gradle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
