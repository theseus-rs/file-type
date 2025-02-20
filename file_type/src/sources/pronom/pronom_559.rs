use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
