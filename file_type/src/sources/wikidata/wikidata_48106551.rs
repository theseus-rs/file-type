use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48106551: FileType = FileType {
    file_format: &FileFormat {
        id: 48_106_551,
        source_type: SourceType::Wikidata,
        name: "DataFlex Query Tag Name",
        extensions: &["tag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
