use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105862164: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_164,
        source_type: SourceType::Wikidata,
        name: "Music Macro Language",
        extensions: &["mml"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
