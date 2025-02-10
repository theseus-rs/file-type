use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51753051: FileType = FileType {
    file_format: &FileFormat {
        id: 51_753_051,
        source_type: SourceType::Wikidata,
        name: "3D Studio Shapes",
        extensions: &["shp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
