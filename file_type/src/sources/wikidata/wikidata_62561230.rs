use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62561230: FileType = FileType {
    file_format: &FileFormat {
        id: 62_561_230,
        source_type: SourceType::Wikidata,
        name: "Corel Presentation, version 3",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
