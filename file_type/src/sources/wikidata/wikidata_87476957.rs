use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87476957: FileFormat = FileFormat {
    id: 87_476_957,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 1",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
