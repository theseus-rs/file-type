use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
