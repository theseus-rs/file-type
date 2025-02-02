use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87481940: FileFormat = FileFormat {
    id: 87_481_940,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 4",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
