use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7339262: FileType = FileType {
    file_format: &FileFormat {
        id: 7_339_262,
        source_type: SourceType::Wikidata,
        name: "RoadXML",
        extensions: &["rnd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
