use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87476961: FileType = FileType {
    file_format: &FileFormat {
        id: 87_476_961,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 2",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
