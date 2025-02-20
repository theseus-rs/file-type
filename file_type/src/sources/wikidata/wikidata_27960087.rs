use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960087: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_087,
        source_type: SourceType::Wikidata,
        name: "Memory Stick Voice",
        extensions: &["msv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
