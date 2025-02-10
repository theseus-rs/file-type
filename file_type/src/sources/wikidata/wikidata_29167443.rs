use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167443: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_443,
        source_type: SourceType::Wikidata,
        name: "OME-TIFF",
        extensions: &["ome.tiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
