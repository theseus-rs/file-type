use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_758: FileType = FileType {
    file_format: &FileFormat {
        id: 758,
        source_type: SourceType::Pronom,
        name: "StarOffice Calc",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
