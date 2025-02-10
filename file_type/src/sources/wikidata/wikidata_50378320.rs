use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50378320: FileType = FileType {
    file_format: &FileFormat {
        id: 50_378_320,
        source_type: SourceType::Wikidata,
        name: "INTERLIS Transfer File, version 2.2",
        extensions: &["xml", "xtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
