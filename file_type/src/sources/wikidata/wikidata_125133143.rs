use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125133143: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_143,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Map file",
        extensions: &["mapx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
