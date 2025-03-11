use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1940878: FileType = FileType {
    file_format: &FileFormat {
        id: 1_940_878,
        source_type: SourceType::Wikidata,
        name: "Mixed Raster Content",
        extensions: &[],
        media_types: &["image/mrc"],
        signatures: &[],
        related_formats: &[],
    },
};
