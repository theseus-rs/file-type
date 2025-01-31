use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59608885: FileFormat = FileFormat {
    id: 59_608_885,
    puid: "wikidata/59608885",
    name: "PKCS #7 Cryptographic message file",
    extensions: &["p7m"],
    media_types: &["application/pkcs7-mime"],
    internal_signatures: &[],
    related_formats: &[],
};
