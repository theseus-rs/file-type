use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1124114: FileType = FileType {
    file_format: &FileFormat {
        id: 1_124_114,
        source_type: SourceType::Wikidata,
        name: "LandXML",
        extensions: &["ddf", "dem", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
