use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109996953: FileType = FileType {
    file_format: &FileFormat {
        id: 109_996_953,
        source_type: SourceType::Wikidata,
        name: "Autocad DMP File",
        extensions: &["dmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
