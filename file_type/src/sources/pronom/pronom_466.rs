use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_466: FileType = FileType {
    file_format: &FileFormat {
        id: 466,
        source_type: SourceType::Pronom,
        name: "Btrieve Database",
        extensions: &["btr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
