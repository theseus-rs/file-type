use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87481940: FileType = FileType {
    file_format: &FileFormat {
        id: 87_481_940,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 4",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
