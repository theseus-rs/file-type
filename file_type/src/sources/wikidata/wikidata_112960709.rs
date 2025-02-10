use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112960709: FileType = FileType {
    file_format: &FileFormat {
        id: 112_960_709,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 environment file",
        extensions: &["gef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
