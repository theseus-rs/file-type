use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87476957: FileType = FileType {
    file_format: &FileFormat {
        id: 87_476_957,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 1",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
