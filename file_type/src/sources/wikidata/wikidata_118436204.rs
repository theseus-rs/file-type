use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118436204: FileType = FileType {
    file_format: &FileFormat {
        id: 118_436_204,
        source_type: SourceType::Wikidata,
        name: "Esri ArcMap Label file",
        extensions: &["lxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
