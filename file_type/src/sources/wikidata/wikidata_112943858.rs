use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112943858: FileType = FileType {
    file_format: &FileFormat {
        id: 112_943_858,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 material definition file",
        extensions: &["gmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
