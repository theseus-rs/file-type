use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_262: FileType = FileType {
    file_format: &FileFormat {
        id: 262,
        source_type: SourceType::Pronom,
        name: "SDSC Image Tool Run-Length Encoded Bitmap",
        extensions: &["rle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
