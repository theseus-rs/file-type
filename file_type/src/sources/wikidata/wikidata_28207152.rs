use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207152: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_152,
        source_type: SourceType::Wikidata,
        name: "PTG",
        extensions: &["ptg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
