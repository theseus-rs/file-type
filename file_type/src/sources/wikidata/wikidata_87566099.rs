use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87566099: FileFormat = FileFormat {
    id: 87_566_099,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 6",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
