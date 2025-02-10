use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000691: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_691,
        source_type: SourceType::Wikidata,
        name: "RayShade Scene Format",
        extensions: &["ray"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
