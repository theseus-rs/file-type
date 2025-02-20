use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
