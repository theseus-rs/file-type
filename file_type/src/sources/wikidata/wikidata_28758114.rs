use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28758114: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_114,
        source_type: SourceType::Wikidata,
        name: "Internet Shortcut",
        extensions: &["url"],
        media_types: &["application/x-mswinurl"],
        signatures: &[],
        related_formats: &[],
    },
};
