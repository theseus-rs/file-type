use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747168: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_168,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Scrollsheet File",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
