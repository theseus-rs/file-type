use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_13: FileType = FileType {
    file_format: &FileFormat {
        id: 13,
        source_type: SourceType::Pronom,
        name: "Online Description Tool Format",
        extensions: &["odt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
