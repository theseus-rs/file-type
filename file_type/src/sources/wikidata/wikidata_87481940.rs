use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87481940: FileFormat = FileFormat {
    id: 87_481_940,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 4",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
