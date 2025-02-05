use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87647627: FileFormat = FileFormat {
    id: 87_647_627,
    source_type: SourceType::Wikidata,
    name: "SketchUp Document 15",
    extensions: &["skb", "skp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
