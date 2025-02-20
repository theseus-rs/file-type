use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66711987: FileType = FileType {
    file_format: &FileFormat {
        id: 66_711_987,
        source_type: SourceType::Wikidata,
        name: "Word Macro-Enabled Template",
        extensions: &["dotm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
