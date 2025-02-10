use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_100: FileType = FileType {
    file_format: &FileFormat {
        id: 100,
        source_type: SourceType::Pronom,
        name: "AutoCAD Last Saved Layer State",
        extensions: &["las"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
