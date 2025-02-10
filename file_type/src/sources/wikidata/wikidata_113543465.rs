use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113543465: FileType = FileType {
    file_format: &FileFormat {
        id: 113_543_465,
        source_type: SourceType::Wikidata,
        name: "Esri Shapefile Geospatial Metadata File",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
