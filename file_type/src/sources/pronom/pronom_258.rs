use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_258: FileType = FileType {
    file_format: &FileFormat {
        id: 258,
        source_type: SourceType::Pronom,
        name: "Raw Bitmap",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
