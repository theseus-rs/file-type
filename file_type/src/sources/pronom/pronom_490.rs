use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_490: FileType = FileType {
    file_format: &FileFormat {
        id: 490,
        source_type: SourceType::Pronom,
        name: "IntelliDraw Vector Graphics",
        extensions: &["idw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
