use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118146092: FileType = FileType {
    file_format: &FileFormat {
        id: 118_146_092,
        source_type: SourceType::Wikidata,
        name: "Edge-coupled symmetric file",
        extensions: &["tl2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
