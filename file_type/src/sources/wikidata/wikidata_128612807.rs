use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128612807: FileFormat = FileFormat {
    id: 128_612_807,
    source_type: SourceType::Wikidata,
    name: "Abstract Syntax Notation One format",
    extensions: &["asn1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
