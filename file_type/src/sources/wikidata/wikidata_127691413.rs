use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127691413: FileType = FileType {
    file_format: &FileFormat {
        id: 127_691_413,
        source_type: SourceType::Wikidata,
        name: "Elm file",
        extensions: &["elm"],
        media_types: &["text/x-elm"],
        signatures: &[],
        related_formats: &[],
    },
};
