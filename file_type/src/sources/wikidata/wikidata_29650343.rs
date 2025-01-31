use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650343: FileFormat = FileFormat {
    id: 29_650_343,
    puid: "wikidata/29650343",
    name: "PEM encoded RSA private key",
    extensions: &["key", "pem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
