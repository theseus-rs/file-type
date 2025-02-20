use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_13543872: FileType = FileType {
    file_format: &FileFormat {
        id: 13_543_872,
        source_type: SourceType::Wikidata,
        name: "Wii ISO Archive",
        extensions: &["wbfs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
