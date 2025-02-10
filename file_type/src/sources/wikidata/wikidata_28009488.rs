use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28009488: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_488,
        source_type: SourceType::Wikidata,
        name: "Tibia map file",
        extensions: &["map"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
