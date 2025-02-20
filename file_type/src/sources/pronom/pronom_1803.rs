use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1803: FileType = FileType {
    file_format: &FileFormat {
        id: 1_803,
        source_type: SourceType::Pronom,
        name: "OpenRaster Image Format",
        extensions: &["ora"],
        media_types: &["image/openraster"],
        signatures: &[],
        related_formats: &[],
    },
};
