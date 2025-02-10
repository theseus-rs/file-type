use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1743: FileType = FileType {
    file_format: &FileFormat {
        id: 1_743,
        source_type: SourceType::Pronom,
        name: "Python Source Code File",
        extensions: &["py"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
