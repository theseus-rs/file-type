use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757904: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_904,
        source_type: SourceType::Wikidata,
        name: "Go script",
        extensions: &["go"],
        media_types: &["text/x-gosrc"],
        signatures: &[],
        related_formats: &[],
    },
};
