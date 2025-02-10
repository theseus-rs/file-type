use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125133233: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_233,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Layout file",
        extensions: &["pagx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
