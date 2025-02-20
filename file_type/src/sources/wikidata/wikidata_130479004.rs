use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130479004: FileType = FileType {
    file_format: &FileFormat {
        id: 130_479_004,
        source_type: SourceType::Wikidata,
        name: "Pointless source code file",
        extensions: &["ptls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
