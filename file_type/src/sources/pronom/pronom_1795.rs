use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1795: FileType = FileType {
    file_format: &FileFormat {
        id: 1_795,
        source_type: SourceType::Pronom,
        name: "ESRI File Geodatabase",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
