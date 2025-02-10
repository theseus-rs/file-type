use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_147: FileType = FileType {
    file_format: &FileFormat {
        id: 147,
        source_type: SourceType::Pronom,
        name: "Schedule+ Contacts",
        extensions: &["scd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
