use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
