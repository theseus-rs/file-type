use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87485941: FileFormat = FileFormat {
    id: 87_485_941,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 5",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
