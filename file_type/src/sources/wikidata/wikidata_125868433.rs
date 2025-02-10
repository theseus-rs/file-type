use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125868433: FileType = FileType {
    file_format: &FileFormat {
        id: 125_868_433,
        source_type: SourceType::Wikidata,
        name: "OpenWayback CDXJ File Format",
        extensions: &["cdx", "cdxj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
