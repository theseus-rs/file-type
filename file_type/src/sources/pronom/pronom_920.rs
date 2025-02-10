use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_920: FileType = FileType {
    file_format: &FileFormat {
        id: 920,
        source_type: SourceType::Pronom,
        name: "ERDAS IMAGINE Gray-scale Bitmap Image",
        extensions: &["gis"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
