use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59608885: FileFormat = FileFormat {
    id: 59_608_885,
    source_type: SourceType::Wikidata,
    name: "PKCS #7 Cryptographic message file",
    extensions: &["p7m"],
    media_types: &["application/pkcs7-mime"],
    signatures: &[],
    related_formats: &[],
};
