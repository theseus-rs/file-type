use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87572414: FileFormat = FileFormat {
    id: 87_572_414,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 13",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
