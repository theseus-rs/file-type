use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116908523: FileType = FileType {
    file_format: &FileFormat {
        id: 116_908_523,
        source_type: SourceType::Wikidata,
        name: "Minecraft data pack",
        extensions: &["zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
