use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87657661: FileType = FileType {
    file_format: &FileFormat {
        id: 87_657_661,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 19",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
