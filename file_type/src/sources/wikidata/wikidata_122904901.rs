use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122904901: FileType = FileType {
    file_format: &FileFormat {
        id: 122_904_901,
        source_type: SourceType::Wikidata,
        name: "PMTiles",
        extensions: &["pmtiles"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
