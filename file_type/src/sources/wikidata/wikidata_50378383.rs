use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50378383: FileType = FileType {
    file_format: &FileFormat {
        id: 50_378_383,
        source_type: SourceType::Wikidata,
        name: "INTERLIS Transfer File, version 1",
        extensions: &["itf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
