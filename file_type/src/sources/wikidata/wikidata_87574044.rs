use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87574044: FileType = FileType {
    file_format: &FileFormat {
        id: 87_574_044,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 14",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
