use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117448874: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_874,
        source_type: SourceType::Wikidata,
        name: "Transcriber TRS Format",
        extensions: &["trs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
