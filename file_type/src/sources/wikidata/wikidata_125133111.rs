use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125133111: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_111,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Project file",
        extensions: &["aprx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
