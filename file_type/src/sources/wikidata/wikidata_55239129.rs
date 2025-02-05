use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55239129: FileFormat = FileFormat {
    id: 55_239_129,
    source_type: SourceType::Wikidata,
    name: "CBOR Web Token format",
    extensions: &["cwt"],
    media_types: &["application/cwt"],
    signatures: &[],
    related_formats: &[],
};
