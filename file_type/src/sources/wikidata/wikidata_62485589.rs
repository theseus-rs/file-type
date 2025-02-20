use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62485589: FileType = FileType {
    file_format: &FileFormat {
        id: 62_485_589,
        source_type: SourceType::Wikidata,
        name: "Navisworks Document, version 2012",
        extensions: &["nwc", "nwd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
