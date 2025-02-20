use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
