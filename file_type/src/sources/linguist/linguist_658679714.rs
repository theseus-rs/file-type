use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_658679714: FileFormat = FileFormat {
    id: 658_679_714,
    source_type: SourceType::Linguist,
    name: "WebVTT",
    extensions: &["vtt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
