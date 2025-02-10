use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207252: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_252,
        source_type: SourceType::Wikidata,
        name: "SCR",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
