use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113161974: FileType = FileType {
    file_format: &FileFormat {
        id: 113_161_974,
        source_type: SourceType::Wikidata,
        name: "Act! database file",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
