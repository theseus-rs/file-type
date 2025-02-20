use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
