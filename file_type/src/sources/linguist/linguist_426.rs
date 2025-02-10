use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_426: FileType = FileType {
    file_format: &FileFormat {
        id: 426,
        source_type: SourceType::Linguist,
        name: "MQL4",
        extensions: &["mq4", "mqh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
