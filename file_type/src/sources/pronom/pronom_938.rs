use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_938: FileType = FileType {
    file_format: &FileFormat {
        id: 938,
        source_type: SourceType::Pronom,
        name: "Information or Setup File",
        extensions: &["inf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
