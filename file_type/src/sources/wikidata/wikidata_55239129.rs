use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55239129: FileFormat = FileFormat {
    id: 55_239_129,
    puid: "wikidata/55239129",
    name: "CBOR Web Token format",
    extensions: &["cwt"],
    media_types: &["application/cwt"],
    internal_signatures: &[],
    related_formats: &[],
};
