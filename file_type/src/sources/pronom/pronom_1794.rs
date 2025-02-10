use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1794: FileType = FileType {
    file_format: &FileFormat {
        id: 1_794,
        source_type: SourceType::Pronom,
        name: "ESRI ArcGlobe Document",
        extensions: &["3dd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
