use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27895562: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_562,
        source_type: SourceType::Wikidata,
        name: "ADX, version 4",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
