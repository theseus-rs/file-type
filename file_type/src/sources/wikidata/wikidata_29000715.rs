use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000715: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_715,
        source_type: SourceType::Wikidata,
        name: "Unreal model data file",
        extensions: &["3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
