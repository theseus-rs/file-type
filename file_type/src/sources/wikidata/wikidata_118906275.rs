use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118906275: FileType = FileType {
    file_format: &FileFormat {
        id: 118_906_275,
        source_type: SourceType::Wikidata,
        name: "ASP Configuration file",
        extensions: &["asa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
