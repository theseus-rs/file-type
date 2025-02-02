use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112875068: FileFormat = FileFormat {
    id: 112_875_068,
    source_type: SourceType::Wikidata,
    name: "armored PGP public key block",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
