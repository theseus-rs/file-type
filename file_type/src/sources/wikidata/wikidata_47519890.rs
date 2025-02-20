use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47519890: FileType = FileType {
    file_format: &FileFormat {
        id: 47_519_890,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 9",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
