use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50378386: FileType = FileType {
    file_format: &FileFormat {
        id: 50_378_386,
        source_type: SourceType::Wikidata,
        name: "INTERLIS Model File, version 1",
        extensions: &["ili"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
