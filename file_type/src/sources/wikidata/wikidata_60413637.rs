use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60413637: FileType = FileType {
    file_format: &FileFormat {
        id: 60_413_637,
        source_type: SourceType::Wikidata,
        name: "INTERLIS Model File, version 2.3",
        extensions: &["ili"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
