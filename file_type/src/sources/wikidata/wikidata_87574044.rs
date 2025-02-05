use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87574044: FileFormat = FileFormat {
    id: 87_574_044,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 14",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
