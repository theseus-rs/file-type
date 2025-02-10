use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100244109: FileType = FileType {
    file_format: &FileFormat {
        id: 100_244_109,
        source_type: SourceType::Wikidata,
        name: "Student Writing Center Newsletter",
        extensions: &["nl", "nlt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
