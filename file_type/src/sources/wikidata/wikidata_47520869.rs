use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47520869: FileType = FileType {
    file_format: &FileFormat {
        id: 47_520_869,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 12",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
