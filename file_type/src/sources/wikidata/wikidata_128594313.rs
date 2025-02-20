use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128594313: FileType = FileType {
    file_format: &FileFormat {
        id: 128_594_313,
        source_type: SourceType::Wikidata,
        name: "Agda file",
        extensions: &["agda"],
        media_types: &["text/x-agda"],
        signatures: &[],
        related_formats: &[],
    },
};
