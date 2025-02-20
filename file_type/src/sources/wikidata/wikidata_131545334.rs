use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131545334: FileType = FileType {
    file_format: &FileFormat {
        id: 131_545_334,
        source_type: SourceType::Wikidata,
        name: "GeoArrow file format",
        extensions: &["arrow"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
