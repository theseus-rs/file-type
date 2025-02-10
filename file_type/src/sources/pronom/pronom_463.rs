use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_463: FileType = FileType {
    file_format: &FileFormat {
        id: 463,
        source_type: SourceType::Pronom,
        name: "Apple Sound",
        extensions: &["afc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
