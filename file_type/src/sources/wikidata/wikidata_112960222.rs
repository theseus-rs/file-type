use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112960222: FileType = FileType {
    file_format: &FileFormat {
        id: 112_960_222,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 camera file",
        extensions: &["gcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
