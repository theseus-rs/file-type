use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87647627: FileType = FileType {
    file_format: &FileFormat {
        id: 87_647_627,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 15",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
