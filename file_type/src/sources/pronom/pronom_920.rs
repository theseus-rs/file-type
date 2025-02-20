use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
