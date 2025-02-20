use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2416: FileType = FileType {
    file_format: &FileFormat {
        id: 2_416,
        source_type: SourceType::Pronom,
        name: "Visual Basic Binary Form File",
        extensions: &["frx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
