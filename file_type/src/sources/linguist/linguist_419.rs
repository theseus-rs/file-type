use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_419: FileFormat = FileFormat {
    id: 419,
    source_type: SourceType::Linguist,
    name: "reStructuredText",
    extensions: &["rest", "rest.txt", "rst", "rst.txt"],
    media_types: &["text/x-rst"],
    signatures: &[],
    related_formats: &[],
};
