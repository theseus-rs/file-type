use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118146513: FileType = FileType {
    file_format: &FileFormat {
        id: 118_146_513,
        source_type: SourceType::Wikidata,
        name: "Coaxial Cable File",
        extensions: &["tl7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
