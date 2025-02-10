use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109623939: FileType = FileType {
    file_format: &FileFormat {
        id: 109_623_939,
        source_type: SourceType::Wikidata,
        name: "WritePlus",
        extensions: &["stt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
