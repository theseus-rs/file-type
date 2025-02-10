use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114978438: FileType = FileType {
    file_format: &FileFormat {
        id: 114_978_438,
        source_type: SourceType::Wikidata,
        name: "StoryView Document",
        extensions: &["syv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
