use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121158405: FileType = FileType {
    file_format: &FileFormat {
        id: 121_158_405,
        source_type: SourceType::Wikidata,
        name: "Merge file",
        extensions: &["mrg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
