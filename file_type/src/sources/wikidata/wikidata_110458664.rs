use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110458664: FileType = FileType {
    file_format: &FileFormat {
        id: 110_458_664,
        source_type: SourceType::Wikidata,
        name: "ERDAS Imagine Large Raster Spill File",
        extensions: &["ige"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
