use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114235996: FileType = FileType {
    file_format: &FileFormat {
        id: 114_235_996,
        source_type: SourceType::Wikidata,
        name: "SYBYL2 format",
        extensions: &["ml2", "sm2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
