use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111355029: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_029,
        source_type: SourceType::Wikidata,
        name: "Unreal Tournament audio package",
        extensions: &["uax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
