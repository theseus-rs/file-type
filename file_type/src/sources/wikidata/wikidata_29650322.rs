use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650322: FileFormat = FileFormat {
    id: 29_650_322,
    puid: "wikidata/29650322",
    name: "PKCS#7 certificate",
    extensions: &[
        "p7b", "p7b", "p7b", "p7c", "p7c", "p7c", "pem", "pem", "pem", "spc", "spc", "spc",
    ],
    media_types: &[
        "application/pkcs7-mime",
        "application/pkcs7-mime",
        "application/pkcs7-mime",
        "application/pkcs7-mime",
        "application/x-pkcs7-certificates",
        "application/x-pkcs7-certificates",
        "application/x-pkcs7-certificates",
        "application/x-pkcs7-certificates",
        "application/x-pkcs7-certreqresp",
        "application/x-pkcs7-certreqresp",
        "application/x-pkcs7-certreqresp",
        "application/x-pkcs7-certreqresp",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
