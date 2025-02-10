use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_19: FileType = FileType {
    file_format: &FileFormat {
        id: 19,
        source_type: SourceType::Linguist,
        name: "AppleScript",
        extensions: &["applescript", "scpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
