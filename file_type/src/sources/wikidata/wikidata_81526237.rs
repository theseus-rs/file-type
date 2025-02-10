use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_81526237: FileType = FileType {
    file_format: &FileFormat {
        id: 81_526_237,
        source_type: SourceType::Wikidata,
        name: "MapInfo MapBasic tabular DataBase",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
