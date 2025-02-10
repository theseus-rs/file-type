use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1502796: FileType = FileType {
    file_format: &FileFormat {
        id: 1_502_796,
        source_type: SourceType::Wikidata,
        name: "GeoTIFF",
        extensions: &["tif"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
