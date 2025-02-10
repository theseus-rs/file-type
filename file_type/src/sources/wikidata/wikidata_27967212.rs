use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967212: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_212,
        source_type: SourceType::Wikidata,
        name: "RASTER Music Tracker module",
        extensions: &["rmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
