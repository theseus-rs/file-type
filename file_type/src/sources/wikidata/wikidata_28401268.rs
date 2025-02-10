use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28401268: FileType = FileType {
    file_format: &FileFormat {
        id: 28_401_268,
        source_type: SourceType::Wikidata,
        name: "XIP",
        extensions: &["xip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
