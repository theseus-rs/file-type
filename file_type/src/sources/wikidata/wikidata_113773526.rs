use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113773526: FileType = FileType {
    file_format: &FileFormat {
        id: 113_773_526,
        source_type: SourceType::Wikidata,
        name: "Painter Raster Image",
        extensions: &["rif", "riff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
