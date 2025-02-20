use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112652505: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_505,
        source_type: SourceType::Wikidata,
        name: "Astound Media Library format",
        extensions: &["mml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
