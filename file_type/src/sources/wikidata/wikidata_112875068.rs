use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112875068: FileFormat = FileFormat {
    id: 112_875_068,
    puid: "wikidata/112875068",
    name: "armored PGP public key block",
    extensions: &["txt", "txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
