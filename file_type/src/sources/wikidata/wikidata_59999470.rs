use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59999470: FileType = FileType {
    file_format: &FileFormat {
        id: 59_999_470,
        source_type: SourceType::Wikidata,
        name: "ESRI Spatial Index File",
        extensions: &["sbn", "sbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
