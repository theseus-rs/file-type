use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125133584: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_584,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Layer package",
        extensions: &["lpkx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
