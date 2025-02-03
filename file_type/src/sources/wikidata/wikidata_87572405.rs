use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87572405: FileFormat = FileFormat {
    id: 87_572_405,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 8",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
