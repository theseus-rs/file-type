use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445584: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_584,
        source_type: SourceType::Wikidata,
        name: "Application Label Data",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
