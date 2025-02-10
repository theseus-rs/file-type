use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
