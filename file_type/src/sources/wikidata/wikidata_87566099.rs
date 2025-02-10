use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87566099: FileType = FileType {
    file_format: &FileFormat {
        id: 87_566_099,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 6",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
