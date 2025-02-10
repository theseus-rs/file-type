use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_559: FileType = FileType {
    file_format: &FileFormat {
        id: 559,
        source_type: SourceType::Pronom,
        name: "Dia Graphics Format",
        extensions: &["dia"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
