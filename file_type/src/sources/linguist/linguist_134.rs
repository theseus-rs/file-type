use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_134: FileFormat = FileFormat {
    id: 134,
    source_type: SourceType::Linguist,
    name: "Gosu",
    extensions: &["gs", "gst", "gsx", "vark"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
