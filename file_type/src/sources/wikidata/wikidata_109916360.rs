use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109916360: FileType = FileType {
    file_format: &FileFormat {
        id: 109_916_360,
        source_type: SourceType::Wikidata,
        name: "JMP Data Table",
        extensions: &["jmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
