use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_936: FileType = FileType {
    file_format: &FileFormat {
        id: 936,
        source_type: SourceType::Pronom,
        name: "Statistica Report File",
        extensions: &["str"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
