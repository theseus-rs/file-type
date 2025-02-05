use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650343: FileFormat = FileFormat {
    id: 29_650_343,
    source_type: SourceType::Wikidata,
    name: "PEM encoded RSA private key",
    extensions: &["key", "pem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
