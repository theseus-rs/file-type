use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125133522: FileType = FileType {
    file_format: &FileFormat {
        id: 125_133_522,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Pro Layer file",
        extensions: &["lyrx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
