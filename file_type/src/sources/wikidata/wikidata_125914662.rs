use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125914662: FileType = FileType {
    file_format: &FileFormat {
        id: 125_914_662,
        source_type: SourceType::Wikidata,
        name: "Sandboxels Save File",
        extensions: &["sbxl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
