use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27996230: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_230,
        source_type: SourceType::Wikidata,
        name: "FileMaker Pro Database, version 5",
        extensions: &["fp5"],
        media_types: &["application/x-filemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
