use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206232: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_232,
        source_type: SourceType::Wikidata,
        name: "HP Paintjet",
        extensions: &["pjx1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
