use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3570443: FileType = FileType {
    file_format: &FileFormat {
        id: 3_570_443,
        source_type: SourceType::Wikidata,
        name: "Xtremsplit file format",
        extensions: &["xtm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
