use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52059869: FileType = FileType {
    file_format: &FileFormat {
        id: 52_059_869,
        source_type: SourceType::Wikidata,
        name: "Micrografx Designer format, version 3.1",
        extensions: &["drw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
