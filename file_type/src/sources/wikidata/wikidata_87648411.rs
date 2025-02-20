use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87648411: FileType = FileType {
    file_format: &FileFormat {
        id: 87_648_411,
        source_type: SourceType::Wikidata,
        name: "SketchUp Document 16",
        extensions: &["skb", "skp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
