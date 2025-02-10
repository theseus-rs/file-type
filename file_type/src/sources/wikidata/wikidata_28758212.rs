use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28758212: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_212,
        source_type: SourceType::Wikidata,
        name: "Street Atlas USA Draw File",
        extensions: &["an1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
