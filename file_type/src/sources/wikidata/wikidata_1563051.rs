use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1563051: FileType = FileType {
    file_format: &FileFormat {
        id: 1_563_051,
        source_type: SourceType::Wikidata,
        name: "HDRi",
        extensions: &["tif", "tiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
