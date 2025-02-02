use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_587855233: FileFormat = FileFormat {
    id: 587_855_233,
    source_type: SourceType::Linguist,
    name: "RON",
    extensions: &["ron"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
