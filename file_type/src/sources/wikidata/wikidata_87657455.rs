use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87657455: FileFormat = FileFormat {
    id: 87_657_455,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 18",
    extensions: &["skb", "skp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
