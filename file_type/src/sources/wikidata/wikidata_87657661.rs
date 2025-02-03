use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87657661: FileFormat = FileFormat {
    id: 87_657_661,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 19",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
