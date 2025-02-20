use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110226037: FileType = FileType {
    file_format: &FileFormat {
        id: 110_226_037,
        source_type: SourceType::Wikidata,
        name: "Raster Matrix Format",
        extensions: &["rsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
