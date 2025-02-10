use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_134: FileType = FileType {
    file_format: &FileFormat {
        id: 134,
        source_type: SourceType::Linguist,
        name: "Gosu",
        extensions: &["gs", "gst", "gsx", "vark"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
