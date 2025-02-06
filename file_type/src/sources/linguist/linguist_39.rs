use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_39: FileFormat = FileFormat {
    id: 39,
    source_type: SourceType::Linguist,
    name: "Brightscript",
    extensions: &["brs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
