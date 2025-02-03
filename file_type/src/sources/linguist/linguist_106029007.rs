use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_106029007: FileFormat = FileFormat {
    id: 106_029_007,
    source_type: SourceType::Linguist,
    name: "Praat",
    extensions: &["praat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
