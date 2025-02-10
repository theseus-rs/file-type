use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87657455: FileType = FileType {
    file_format: &FileFormat {
        id: 87_657_455,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 18",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
