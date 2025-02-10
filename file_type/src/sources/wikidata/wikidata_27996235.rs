use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27996235: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_235,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 3",
        extensions: &["fp3"],
        media_types: &["application/x-filemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
