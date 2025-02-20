use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
