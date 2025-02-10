use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_427: FileType = FileType {
    file_format: &FileFormat {
        id: 427,
        source_type: SourceType::Linguist,
        name: "MQL5",
        extensions: &["mq5", "mqh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
