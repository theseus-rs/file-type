use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133897750: FileType = FileType {
    file_format: &FileFormat {
        id: 133_897_750,
        source_type: SourceType::Wikidata,
        name: "ArcGIS Layer file",
        extensions: &["lyr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
