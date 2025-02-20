use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87481529: FileType = FileType {
    file_format: &FileFormat {
        id: 87_481_529,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 3",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
