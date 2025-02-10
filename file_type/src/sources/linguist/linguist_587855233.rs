use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_587855233: FileType = FileType {
    file_format: &FileFormat {
        id: 587_855_233,
        source_type: SourceType::Linguist,
        name: "RON",
        extensions: &["ron"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
