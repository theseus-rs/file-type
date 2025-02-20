use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112943826: FileType = FileType {
    file_format: &FileFormat {
        id: 112_943_826,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 raw object 'body' definition file",
        extensions: &["gbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
