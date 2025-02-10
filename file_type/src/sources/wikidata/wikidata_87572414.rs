use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87572414: FileType = FileType {
    file_format: &FileFormat {
        id: 87_572_414,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 13",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
