use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207158: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_158,
        source_type: SourceType::Wikidata,
        name: "Puzzle image",
        extensions: &["cm", "pzl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
