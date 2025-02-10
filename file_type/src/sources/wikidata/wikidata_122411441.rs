use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122411441: FileType = FileType {
    file_format: &FileFormat {
        id: 122_411_441,
        source_type: SourceType::Wikidata,
        name: "DWARF Symbolic File",
        extensions: &["dwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
