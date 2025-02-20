use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_520: FileType = FileType {
    file_format: &FileFormat {
        id: 520,
        source_type: SourceType::Pronom,
        name: "SAP Document",
        extensions: &["ali"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
