use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87485941: FileType = FileType {
    file_format: &FileFormat {
        id: 87_485_941,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 5",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
