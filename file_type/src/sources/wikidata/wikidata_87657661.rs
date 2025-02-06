use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87657661: FileFormat = FileFormat {
    id: 87_657_661,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 19",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
