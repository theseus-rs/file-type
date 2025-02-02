use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_76: FileFormat = FileFormat {
    id: 76,
    source_type: SourceType::Linguist,
    name: "Gherkin",
    extensions: &["feature", "story"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
