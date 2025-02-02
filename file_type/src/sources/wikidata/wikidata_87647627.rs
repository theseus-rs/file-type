use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87647627: FileFormat = FileFormat {
    id: 87_647_627,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 15",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
