use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125133556: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_556,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Map package",
        extensions: &["mpkx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
