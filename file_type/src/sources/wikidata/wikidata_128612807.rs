use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128612807: FileFormat = FileFormat {
    id: 128_612_807,
    puid: "wikidata/128612807",
    name: "Abstract Syntax Notation One format",
    extensions: &["asn1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
