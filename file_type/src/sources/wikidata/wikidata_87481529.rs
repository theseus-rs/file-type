use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87481529: FileFormat = FileFormat {
    id: 87_481_529,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 3",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
