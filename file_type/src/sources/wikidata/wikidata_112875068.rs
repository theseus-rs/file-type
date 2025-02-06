use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112875068: FileFormat = FileFormat {
    id: 112_875_068,
    source_type: SourceType::Wikidata,
    name: "armored PGP public key block",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
