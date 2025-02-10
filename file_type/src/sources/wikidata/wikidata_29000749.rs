use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000749: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_749,
        source_type: SourceType::Wikidata,
        name: "YASRT Raytracer Scene",
        extensions: &["yst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
