use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_117: FileType = FileType {
    file_format: &FileFormat {
        id: 117,
        source_type: SourceType::Linguist,
        name: "G-code",
        extensions: &["cnc", "g", "gco", "gcode"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
