use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123378450: FileType = FileType {
    file_format: &FileFormat {
        id: 123_378_450,
        source_type: SourceType::Wikidata,
        name: "TrueSpace Selection file",
        extensions: &["sel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
