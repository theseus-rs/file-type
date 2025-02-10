use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52006189: FileType = FileType {
    file_format: &FileFormat {
        id: 52_006_189,
        source_type: SourceType::Wikidata,
        name: "Micrografx Draw, version 4",
        extensions: &["drw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
