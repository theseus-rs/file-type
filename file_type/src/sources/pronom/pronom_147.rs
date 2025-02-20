use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
