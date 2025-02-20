use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
