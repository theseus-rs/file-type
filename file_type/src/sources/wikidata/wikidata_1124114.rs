use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
