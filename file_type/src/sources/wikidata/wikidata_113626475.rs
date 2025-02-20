use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113626475: FileType = FileType {
    file_format: &FileFormat {
        id: 113_626_475,
        source_type: SourceType::Wikidata,
        name: "FOCUS file",
        extensions: &["fex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
