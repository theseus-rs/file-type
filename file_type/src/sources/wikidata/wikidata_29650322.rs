use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650322: FileFormat = FileFormat {
    id: 29_650_322,
    source_type: SourceType::Wikidata,
    name: "PKCS#7 certificate",
    extensions: &["p7b", "p7c", "pem", "spc"],
    media_types: &[
        "application/pkcs7-mime",
        "application/x-pkcs7-certificates",
        "application/x-pkcs7-certreqresp",
    ],
    signatures: &[],
    related_formats: &[],
};
