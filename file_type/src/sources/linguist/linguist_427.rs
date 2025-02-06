use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_427: FileFormat = FileFormat {
    id: 427,
    source_type: SourceType::Linguist,
    name: "MQL5",
    extensions: &["mq5", "mqh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
