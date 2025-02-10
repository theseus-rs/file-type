use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28445589: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_589,
        source_type: SourceType::Wikidata,
        name: "AMOS AmBs",
        extensions: &["abk", "abs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
