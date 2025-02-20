use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109997309: FileType = FileType {
    file_format: &FileFormat {
        id: 109_997_309,
        source_type: SourceType::Wikidata,
        name: "Stuffit Archive File, version 1.5",
        extensions: &["sit"],
        media_types: &["application/x-stuffit"],
        signatures: &[],
        related_formats: &[],
    },
};
