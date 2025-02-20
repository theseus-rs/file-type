use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118140141: FileType = FileType {
    file_format: &FileFormat {
        id: 118_140_141,
        source_type: SourceType::Wikidata,
        name: "Serenade Schematic File",
        extensions: &["sch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
