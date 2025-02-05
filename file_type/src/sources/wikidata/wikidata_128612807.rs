use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128612807: FileFormat = FileFormat {
    id: 128_612_807,
    source_type: SourceType::Wikidata,
    name: "Abstract Syntax Notation One format",
    extensions: &["asn1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
