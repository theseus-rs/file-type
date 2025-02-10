use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1284: FileType = FileType {
    file_format: &FileFormat {
        id: 1_284,
        source_type: SourceType::Pronom,
        name: "Wireless Bitmap",
        extensions: &["wbmp"],
        media_types: &["image/vnd-wap-wbmp"],
        signatures: &[],
        related_formats: &[],
    },
};
