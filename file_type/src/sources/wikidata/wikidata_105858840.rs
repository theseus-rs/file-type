use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105858840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_840,
        source_type: SourceType::Wikidata,
        name: "Chrome Bookmarks",
        extensions: &[],
        media_types: &["text/json"],
        signatures: &[],
        related_formats: &[],
    },
};
