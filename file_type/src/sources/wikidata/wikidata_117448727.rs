use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117448727: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_727,
        source_type: SourceType::Wikidata,
        name: "Transcriber AG TAG Format",
        extensions: &["tag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
