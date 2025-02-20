use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62561203: FileType = FileType {
    file_format: &FileFormat {
        id: 62_561_203,
        source_type: SourceType::Wikidata,
        name: "Corel Presentation",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
