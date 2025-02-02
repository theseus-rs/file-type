use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_200: FileFormat = FileFormat {
    id: 200,
    source_type: SourceType::Linguist,
    name: "LilyPond",
    extensions: &["ily", "ly"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
