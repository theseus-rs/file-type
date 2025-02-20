use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118584012: FileType = FileType {
    file_format: &FileFormat {
        id: 118_584_012,
        source_type: SourceType::Wikidata,
        name: "Cakewalk Template",
        extensions: &["cwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
