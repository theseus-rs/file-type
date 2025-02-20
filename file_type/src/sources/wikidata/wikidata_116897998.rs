use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116897998: FileType = FileType {
    file_format: &FileFormat {
        id: 116_897_998,
        source_type: SourceType::Wikidata,
        name: "Minecraft resource pack",
        extensions: &["zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
