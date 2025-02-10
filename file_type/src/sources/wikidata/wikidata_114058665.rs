use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114058665: FileType = FileType {
    file_format: &FileFormat {
        id: 114_058_665,
        source_type: SourceType::Wikidata,
        name: "Canon SIF File",
        extensions: &["sif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
