use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122904069: FileType = FileType {
    file_format: &FileFormat {
        id: 122_904_069,
        source_type: SourceType::Wikidata,
        name: "MBTiles",
        extensions: &["mbtiles"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
