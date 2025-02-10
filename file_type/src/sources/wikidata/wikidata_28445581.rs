use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28445581: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_581,
        source_type: SourceType::Wikidata,
        name: "ADRIFT",
        extensions: &["taf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
