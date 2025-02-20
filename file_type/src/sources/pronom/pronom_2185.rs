use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2185: FileType = FileType {
    file_format: &FileFormat {
        id: 2_185,
        source_type: SourceType::Pronom,
        name: "GeoJSON",
        extensions: &["geojson"],
        media_types: &["application/geo+json"],
        signatures: &[],
        related_formats: &[],
    },
};
