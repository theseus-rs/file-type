use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114409: FileFormat = FileFormat {
    id: 114_409,
    source_type: SourceType::Wikidata,
    name: "Turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    signatures: &[],
    related_formats: &[],
};
