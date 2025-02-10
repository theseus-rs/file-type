use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113162065: FileType = FileType {
    file_format: &FileFormat {
        id: 113_162_065,
        source_type: SourceType::Wikidata,
        name: "Approach database file",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
