use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109997611: FileType = FileType {
    file_format: &FileFormat {
        id: 109_997_611,
        source_type: SourceType::Wikidata,
        name: "Stuffit Archive File, version 1.6-4.5",
        extensions: &["sit"],
        media_types: &["application/x-stuffit"],
        signatures: &[],
        related_formats: &[],
    },
};
