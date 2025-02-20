use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_248: FileType = FileType {
    file_format: &FileFormat {
        id: 248,
        source_type: SourceType::Pronom,
        name: "MacPaint Graphics",
        extensions: &["pnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
