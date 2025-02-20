use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118146053: FileType = FileType {
    file_format: &FileFormat {
        id: 118_146_053,
        source_type: SourceType::Wikidata,
        name: "Microstrip File",
        extensions: &["tl1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
