use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7670377: FileType = FileType {
    file_format: &FileFormat {
        id: 7_670_377,
        source_type: SourceType::Wikidata,
        name: "Tagged Image File Format/Electronic Photography",
        extensions: &["tif", "tiff"],
        media_types: &["image/tiff"],
        signatures: &[],
        related_formats: &[],
    },
};
