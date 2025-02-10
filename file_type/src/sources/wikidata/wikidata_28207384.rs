use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207384: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_384,
        source_type: SourceType::Wikidata,
        name: "TIFF/IT",
        extensions: &["tif", "tiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
