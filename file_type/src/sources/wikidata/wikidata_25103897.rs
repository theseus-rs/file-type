use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_25103897: FileType = FileType {
    file_format: &FileFormat {
        id: 25_103_897,
        source_type: SourceType::Wikidata,
        name: "Dynamic Text Document",
        extensions: &["dtxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
