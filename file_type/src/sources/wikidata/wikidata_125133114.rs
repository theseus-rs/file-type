use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125133114: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_114,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Project Package",
        extensions: &["ppkx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
