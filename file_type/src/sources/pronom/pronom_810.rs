use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_810: FileType = FileType {
    file_format: &FileFormat {
        id: 810,
        source_type: SourceType::Pronom,
        name: "JavaScript file",
        extensions: &["js"],
        media_types: &["application/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
