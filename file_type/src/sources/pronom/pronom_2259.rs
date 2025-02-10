use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2259: FileType = FileType {
    file_format: &FileFormat {
        id: 2_259,
        source_type: SourceType::Pronom,
        name: "Apple iWork Document",
        extensions: &["iwa", "key", "pages", "numbers", "template"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
