use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207389: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_389,
        source_type: SourceType::Wikidata,
        name: "TIM",
        extensions: &["tim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
