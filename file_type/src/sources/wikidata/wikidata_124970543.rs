use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124970543: FileType = FileType {
    file_format: &FileFormat {
        id: 124_970_543,
        source_type: SourceType::Wikidata,
        name: "MIX message data file",
        extensions: &["mix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
