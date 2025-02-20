use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125133635: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_635,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Project Template",
        extensions: &["aptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
