use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114130153: FileType = FileType {
    file_format: &FileFormat {
        id: 114_130_153,
        source_type: SourceType::Wikidata,
        name: "Camtasia Producer Project",
        extensions: &["cam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
