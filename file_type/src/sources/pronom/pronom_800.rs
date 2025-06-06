use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_800: FileType = FileType {
    file_format: &FileFormat {
        id: 800,
        source_type: SourceType::Pronom,
        name: "Batch file (executable)",
        extensions: &["bat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
