use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59537303: FileType = FileType {
    file_format: &FileFormat {
        id: 59_537_303,
        source_type: SourceType::Wikidata,
        name: "Nullsoft Scriptable Install System",
        extensions: &["nsi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
