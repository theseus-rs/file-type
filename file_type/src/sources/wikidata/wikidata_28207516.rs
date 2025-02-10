use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207516: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_516,
        source_type: SourceType::Wikidata,
        name: "Word for DOS screen capture",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
