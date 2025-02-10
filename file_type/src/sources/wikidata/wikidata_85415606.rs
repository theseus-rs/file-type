use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85415606: FileType = FileType {
    file_format: &FileFormat {
        id: 85_415_606,
        source_type: SourceType::Wikidata,
        name: "Sonic Scenarist Closed Caption Format",
        extensions: &["scc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
