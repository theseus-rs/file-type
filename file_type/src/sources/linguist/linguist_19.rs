use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_19: FileFormat = FileFormat {
    id: 19,
    source_type: SourceType::Linguist,
    name: "AppleScript",
    extensions: &["applescript", "scpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
