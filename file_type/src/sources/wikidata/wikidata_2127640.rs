use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2127640: FileType = FileType {
    file_format: &FileFormat {
        id: 2_127_640,
        source_type: SourceType::Wikidata,
        name: "Railway Markup Language",
        extensions: &["railml", "railmlx"],
        media_types: &["application/xml", "application/zip", "text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
