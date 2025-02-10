use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27895570: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_570,
        source_type: SourceType::Wikidata,
        name: "ADX, version 5",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
