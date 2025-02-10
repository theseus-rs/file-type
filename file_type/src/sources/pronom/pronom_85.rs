use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_85: FileType = FileType {
    file_format: &FileFormat {
        id: 85,
        source_type: SourceType::Pronom,
        name: "Drawing Interchange Format Style Extract",
        extensions: &["dxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
