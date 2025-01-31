use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205667: FileFormat = FileFormat {
    id: 28_205_667,
    puid: "wikidata/28205667",
    name: "Public Key Cryptography Standard 10",
    extensions: &["csr", "p10", "pem"],
    media_types: &[
        "application/pkcs10",
        "application/pkcs10",
        "application/pkcs10",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
