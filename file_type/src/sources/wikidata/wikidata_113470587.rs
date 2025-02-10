use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113470587: FileType = FileType {
    file_format: &FileFormat {
        id: 113_470_587,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcGIS Raw Raster Reader/ Writer",
        extensions: &["hdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
