use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87476961: FileFormat = FileFormat {
    id: 87_476_961,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 2",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
