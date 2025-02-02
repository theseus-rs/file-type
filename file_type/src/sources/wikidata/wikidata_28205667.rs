use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205667: FileFormat = FileFormat {
    id: 28_205_667,
    source_type: SourceType::Wikidata,
    name: "Public Key Cryptography Standard 10",
    extensions: &["csr", "p10", "pem"],
    media_types: &["application/pkcs10"],
    internal_signatures: &[],
    related_formats: &[],
};
