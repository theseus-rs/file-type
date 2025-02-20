use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87654419: FileType = FileType {
    file_format: &FileFormat {
        id: 87_654_419,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 17",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
