use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87648411: FileFormat = FileFormat {
    id: 87_648_411,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 16",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
