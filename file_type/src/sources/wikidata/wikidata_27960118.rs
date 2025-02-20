use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960118: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_118,
        source_type: SourceType::Wikidata,
        name: "Sony Wave64",
        extensions: &["w64"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
