use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112943753: FileType = FileType {
    file_format: &FileFormat {
        id: 112_943_753,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 GRP file",
        extensions: &["grp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
