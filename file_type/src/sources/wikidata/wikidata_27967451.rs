use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967451: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_451,
        source_type: SourceType::Wikidata,
        name: "GRASP GL",
        extensions: &["gl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
