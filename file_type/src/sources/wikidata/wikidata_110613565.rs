use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110613565: FileType = FileType {
    file_format: &FileFormat {
        id: 110_613_565,
        source_type: SourceType::Wikidata,
        name: "Mapbox Vector Tiles",
        extensions: &["mvt"],
        media_types: &["application/vnd.mapbox-vector-tile"],
        signatures: &[],
        related_formats: &[],
    },
};
