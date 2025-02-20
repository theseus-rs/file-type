use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87572405: FileType = FileType {
    file_format: &FileFormat {
        id: 87_572_405,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 8",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
