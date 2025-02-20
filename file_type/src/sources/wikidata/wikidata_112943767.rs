use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112943767: FileType = FileType {
    file_format: &FileFormat {
        id: 112_943_767,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 raw object definition file",
        extensions: &["gof"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
