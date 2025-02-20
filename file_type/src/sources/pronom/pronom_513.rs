use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_513: FileType = FileType {
    file_format: &FileFormat {
        id: 513,
        source_type: SourceType::Pronom,
        name: "Nota Bene Text File",
        extensions: &["nb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
