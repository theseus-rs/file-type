use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132540526: FileType = FileType {
    file_format: &FileFormat {
        id: 132_540_526,
        source_type: SourceType::Wikidata,
        name: "Tile Drain Properties file format",
        extensions: &["tprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
